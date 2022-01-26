use async_trait::async_trait;
use chrono::format;
use diesel::sql_query;
use std::sync::Arc;

use crate::diesel::prelude::*;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::diesel::{insert_into, update};

use crate::connection::PgPool;
use crate::connection::PgPooledConnection;
use crate::schema::game_match_event::event_type;

// type and structure imports
use super::repo::Repo;
use crate::db_models::{
    game_match::{CreateGameMatch, GameMatch},
    game_match_event::{CreateGameMatchEvent, GameMatchEvent, GameMatchEventType},
};
use crate::result_types::GameMatchShow;

// schema imports
use crate::schema::{game, game_match, game_match_event, team, team_plays_game};

/// Structure containing a reference to a database connection pool
/// and methods to access the database
/// to work with Match records
pub struct PgMatchRepo {
    pub pool: Arc<PgPool>,
}

#[async_trait]
impl Repo for PgMatchRepo {
    /// Create a new Team repo with a reference to an initialized pool.
    ///
    /// Params
    /// ---
    /// - pool: A reference to an already initialized database connection pool,
    ///         used for connecting to the database
    ///
    /// Returns
    /// ---
    /// - new Team repo
    fn new(pool: &Arc<PgPool>) -> PgMatchRepo {
        PgMatchRepo {
            pool: Arc::clone(pool),
        }
    }

    /// Get a connection from the pool
    ///
    /// Returns
    /// ---
    /// - Ok(pooled_connection) if no error occurs
    /// - Err(_) if the wait for another connection is too long
    async fn get_connection(&self) -> anyhow::Result<PgPooledConnection> {
        Ok(self.pool.get()?)
    }
}

#[async_trait]
pub trait MatchRepo {
    /// Get a desired match by its ID
    ///
    /// Params
    /// ---
    /// - desired_match_id: ID of the desired match
    ///
    /// Returns
    /// ---
    /// - Ok(GameMatch) if the match has been found
    /// - Err(_) if an error has occurred or the match has not been found
    async fn get(&self, desired_match_id: i32) -> anyhow::Result<GameMatch>;

    /// Get a desired match (together with display information) by its ID
    ///
    /// Params
    /// ---
    /// - desired_match_id: ID of the desired match
    ///
    /// Returns
    /// ---
    /// - Ok(GameMatchShow) if the match has been found
    /// - Err(_) if an error has occurred or the match has not been found
    async fn get_show_info(&self, desired_match_id: i32) -> anyhow::Result<GameMatchShow>;

    /// Get all matches, optionally we can filter by the time period (upcoming),
    /// filter by the team and filter by the game
    ///
    /// Params
    /// ---
    /// - filter_by_time_period: select the time period we wish to filter by
    /// - filter_by_game: select the game we wish to retrieve the matches of
    /// (the filters will be combined (as if you wrote AND between them))
    ///
    /// Returns
    /// ---
    /// - Ok(Vec<GameMatch>) on successful matches retrieval
    /// - Err(_) if an error has occurred
    async fn get_all_show(
        &self,
        filter_by_time_period: Option<GameMatchEventType>,
        filter_by_game: Option<i32>,
        // filter_by_team: Option<i32>,
    ) -> anyhow::Result<Vec<GameMatchShow>>;

    /// Create a new game match structure (and set the latest event of the match to upcoming)
    /// Additionally, it checks that both teams are playing the chosen game
    ///
    /// Params
    /// ---
    /// - new_match: write structure for creating a new game match
    ///
    /// Returns
    /// ---
    /// - Ok(id) if the match could be created -> both teams exist, game exists and both teams play the game
    /// - Err(_) if an error occurrs or some bounds were not satisfied
    async fn create<'a>(&self, new_match: CreateGameMatch<'a>) -> anyhow::Result<i32>;

    /// Update the state for chosen match
    ///
    /// Params
    /// ---
    /// - desired_match_id: ID of the match
    /// - desired_new_state: what to update the state to
    ///
    /// Returns
    /// ---
    /// - Ok(()) if the update was successful
    /// - Err(_) if an error has occurred
    async fn update_match_state(
        &self,
        desired_match_id: i32,
        desired_new_state: String,
    ) -> anyhow::Result<()>;

    /// Create an event for the match
    /// Fails if such event already exists
    ///
    /// Params
    /// ---
    /// - desired_match_id: ID of the match we want to create an event for
    /// - desired_event_type: the event we wish to store
    ///
    /// Returns
    /// ---
    /// - Ok(()) if the event has been created successfully
    async fn create_event(
        &self,
        desired_match_id: i32,
        desired_event_type: GameMatchEventType,
    ) -> anyhow::Result<i32>;

    /// Edit an event
    /// It is possible to edit events -> Live, Overtime
    ///
    /// Params
    /// ---
    /// - desired_match_id: ID of the match we need to edit the events of
    /// - which_event: Which event type we want to target
    /// - new_value: new value of the event that will be updated in the database
    ///
    /// Returns
    /// ---
    /// - Ok(()) if the event has been updated successfully
    /// - Err(_) if an error occurred, or a wrong event_type has been selected, or the event does not exist
    async fn edit_event(
        &self,
        desired_match_id: i32,
        which_event: GameMatchEventType,
        new_value: CreateGameMatchEvent,
    ) -> anyhow::Result<()>;

    /// Delete an event type -> in special cases, we wish to delete an event
    /// Supported event types -> all except the `Upcoming` type
    ///
    /// Params
    /// ---
    /// - desired_match_id: ID of the match we need to delete the event of
    /// - which_event: what event type
    async fn delete_event(&self, desired_match_id: i32, which_event: GameMatchEventType);
}

#[async_trait]
impl MatchRepo for PgMatchRepo {
    /// Get a desired match by its ID
    async fn get(&self, desired_match_id: i32) -> anyhow::Result<GameMatch> {
        let query_result: GameMatch = game_match::table
            .find(desired_match_id)
            .get_result(&self.get_connection().await?)?;

        Ok(query_result)
    }

    /// Get a desired match (together with display information) by its ID
    async fn get_show_info(&self, desired_match_id: i32) -> anyhow::Result<GameMatchShow> {
        let query = sql_query(format!(
            "
        SELECT game_match.*
            , match_event.event_type
            , team_one.team_one_name
            , team_two.team_two_name
            , game_table.games_name
                FROM game_match
                INNER JOIN (SELECT DISTINCT ON (game_match_id) game_match_id AS event_match_id
                                    , event_type
                                    , created_at AS event_created_at
                            FROM game_match_event
                            ORDER BY event_match_id, event_created_at DESC
                            ) AS match_event
                ON game_match.id=match_event.event_match_id
                INNER JOIN (SELECT id AS first_team_id
                                    , name AS team_one_name
                            FROM team) AS team_one
                ON game_match.team_one_id=team_one.first_team_id
                INNER JOIN (SELECT id AS second_team_id
                                    , name AS team_two_name
                            FROM team) AS team_two
                ON game_match.team_two_id=team_two.second_team_id
                INNER JOIN (SELECT id AS games_id
                                    , name AS games_name
                            FROM game) AS game_table
                ON game_match.game_id=game_table.games_id
                WHERE game_match.id={}",
            desired_match_id
        ));

        let query_result: GameMatchShow = query.get_result(&self.get_connection().await?)?;

        Ok(query_result)
    }

    /// Get all matches, optionally we can filter by the time period (upcoming),
    /// filter by the team and filter by the game
    async fn get_all_show(
        &self,
        filter_by_time_period: Option<GameMatchEventType>,
        filter_by_game: Option<i32>,
        // filter_by_team: Option<i32>,
    ) -> anyhow::Result<Vec<GameMatchShow>> {
        // raw query performing all necessarry joins -> could not do it with Diesel

        // optional parts of the clause need to be addressed:
        // adding a where clause if there is some sort of a filter
        let where_clause = if filter_by_game.is_some() || filter_by_time_period.is_some() {
            "WHERE"
        } else {
            ""
        };
        // adding an `and` clause if there are two optional filters
        let and_clause = if filter_by_game.is_some() && filter_by_time_period.is_some() {
            "AND"
        } else {
            ""
        };

        // filter string -> possibility to expand the options
        let filter_string = format!(
            "{} {} {} {}",
            where_clause,
            filter_by_time_period.map_or_else(
                || "".into(),
                |time_period: GameMatchEventType| {
                    format!("match_event.event_type='{}'", time_period)
                }
            ),
            and_clause,
            filter_by_game.map_or_else(
                || "".into(),
                |game_id: i32| { format!("game_match.game_id={}", game_id) }
            )
        );

        // final query string with the filter at the end
        let query_string = format!(
            "
        SELECT game_match.*
        , match_event.event_type
        , team_one.team_one_name
        , team_two.team_two_name
        , game_table.games_name
            FROM game_match
            INNER JOIN (SELECT DISTINCT ON (game_match_id) game_match_id AS event_match_id
                                , event_type
                                , created_at AS event_created_at
                        FROM game_match_event
                        ORDER BY event_match_id, event_created_at DESC
                        ) AS match_event
            ON game_match.id=match_event.event_match_id
            INNER JOIN (SELECT id AS first_team_id
                                , name AS team_one_name
                        FROM team) AS team_one
            ON game_match.team_one_id=team_one.first_team_id
            INNER JOIN (SELECT id AS second_team_id
                                , name AS team_two_name
                        FROM team) AS team_two
            ON game_match.team_two_id=team_two.second_team_id
            INNER JOIN (SELECT id AS games_id
                                , name AS games_name
                        FROM game) AS game_table
            ON game_match.game_id=game_table.games_id
            {}
        ",
            filter_string
        );

        let result: Vec<GameMatchShow> =
            sql_query(query_string).get_results(&self.get_connection().await?)?;

        Ok(result)
    }

    /// Create a new game match structure (and set the latest event of the match to upcoming)
    /// Additionally, it checks that both teams are playing the chosen game
    async fn create<'a>(&self, new_match: CreateGameMatch<'a>) -> anyhow::Result<i32> {
        let connection: PgPooledConnection = self.get_connection().await?;
        // Check if both teams are playing the game
        let both_teams_play_the_game: usize = team_plays_game::table
            .filter(
                team_plays_game::team_id
                    .eq(new_match.team_one_id)
                    .and(team_plays_game::game_id.eq(new_match.game_id)),
            )
            .or_filter(
                team_plays_game::team_id
                    .eq(new_match.team_two_id)
                    .and(team_plays_game::game_id.eq(new_match.game_id)),
            )
            .execute(&connection)?;

        // possible results
        match both_teams_play_the_game {
            0 => anyhow::bail!("None of the teams selected is playing this game"),
            1 => anyhow::bail!("One of the teams selected is not playing this game"),
            2 => {}
            _ => anyhow::bail!("Internal error!!! More occurrences of the same team playing the game multiple times!"),
        }

        // create the game match
        let query_result: i32 = insert_into(game_match::table)
            .values(new_match)
            .returning(game_match::id)
            .get_result(&connection)?;

        // create an upcoming event for the new match
        self.create_event(query_result, GameMatchEventType::Upcoming)
            .await?;

        Ok(query_result)
    }

    /// Update the state for chosen match
    async fn update_match_state(
        &self,
        desired_match_id: i32,
        desired_new_state: String,
    ) -> anyhow::Result<()> {
        let _ = update(game_match::table.find(desired_match_id))
            .set(game_match::state.eq(desired_new_state))
            .execute(&self.get_connection().await?)?;

        Ok(())
    }

    /// Create an event for the match
    /// Fails if such event already exists
    async fn create_event(
        &self,
        desired_match_id: i32,
        desired_event_type: GameMatchEventType,
    ) -> anyhow::Result<i32> {
        // obtain connection
        let connection: PgPooledConnection = self.get_connection().await?;

        // find if an event already exists
        let exists: usize = game_match_event::table
            .filter(
                game_match_event::game_match_id
                    .eq(desired_match_id)
                    .and(game_match_event::event_type.eq(desired_event_type.to_string())),
            )
            .execute(&connection)?;

        // what to do with an already existing event?
        match exists {
            0 => {}
            1 => anyhow::bail!("The event you wish to create already exists!"),
            _ => anyhow::bail!("Internal error! More than 1 events of the same type exist"),
        }

        // create an event
        let query_result: i32 = insert_into(game_match_event::table)
            .values(CreateGameMatchEvent::new(
                desired_match_id,
                desired_event_type,
            ))
            .returning(game_match_event::id)
            .get_result(&connection)?;

        Ok(query_result)
    }

    /// Edit an event
    /// It is possible to edit events -> Live, Overtime
    async fn edit_event(
        &self,
        desired_match_id: i32,
        which_event: GameMatchEventType,
        new_value: CreateGameMatchEvent,
    ) -> anyhow::Result<()> {
        todo!()
    }

    /// Delete an event type -> in special cases, we wish to delete an event
    /// Supported event types -> all except the `Upcoming` type
    async fn delete_event(&self, desired_match_id: i32, which_event: GameMatchEventType) {
        todo!()
    }
}
