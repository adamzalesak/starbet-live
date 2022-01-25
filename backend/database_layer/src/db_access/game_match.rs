use async_trait::async_trait;
use chrono::format;
use diesel::sql_query;
use std::sync::Arc;

use crate::diesel::insert_into;
use crate::diesel::prelude::*;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;

use crate::connection::PgPool;
use crate::connection::PgPooledConnection;

// type and structure imports
use super::repo::Repo;
use crate::db_models::{
    game_match::{CreateGameMatch, GameMatch},
    game_match_event::{CreateGameMatchEvent, GameMatchEvent, GameMatchEventType},
};
use crate::result_types::GameMatchShow;

// schema imports
use crate::schema::{game, game_match, game_match_event, team};

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

    async fn create<'a>(&self, new_match: CreateGameMatch<'a>);

    async fn update_game_state(
        &self,
        desired_match_id: i32,
        desired_new_state: String,
    ) -> anyhow::Result<()>;

    async fn create_event(&self, desired_match_id: i32) -> anyhow::Result<()>;
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

    async fn create<'a>(&self, new_match: CreateGameMatch<'a>) {
        todo!()
    }

    async fn update_game_state(
        &self,
        desired_match_id: i32,
        desired_new_state: String,
    ) -> anyhow::Result<()> {
        todo!()
    }

    async fn create_event(&self, desired_match_id: i32) -> anyhow::Result<()> {
        todo!()
    }
}
