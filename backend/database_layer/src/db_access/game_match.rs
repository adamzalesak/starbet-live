use async_trait::async_trait;
use std::sync::Arc;

use crate::connection::{PgPool, PgPooledConnection};
use crate::diesel::{delete, insert_into, prelude::*, sql_query, update, QueryDsl, RunQueryDsl};
use crate::type_storing::time_handling::TimeHandling;
use chrono::{DateTime, Duration, Utc};

// type and structure imports
use super::repo::Repo;
use crate::db_models::{
    game_match::{CreateGameMatch, GameMatch, GameMatchUpdate},
    game_match_event::{
        CreateGameMatchEvent, GameMatchEvent, GameMatchEventFilter, GameMatchEventType,
    },
};

// schema imports
use crate::schema::{
    bet, game, game_match, game_match_event, submitted_bet, team, team_plays_game,
};

/// Structure containing a reference to a database connection pool
/// and methods to access the database
/// to work with GameMatch records
pub struct PgMatchRepo {
    pub pool: Arc<PgPool>,
}

#[async_trait]
impl Repo for PgMatchRepo {
    /// Create a new Match repo with a reference to an initialized pool.
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
    async fn create(&self, new_match: CreateGameMatch) -> anyhow::Result<i32>;

    /// Delete a game match
    /// Only possible if the match is "`Upcoming`" and there are no bets tied to the match yet.
    /// Deleting the match does not work, if the match is about to start in 5 seconds
    ///
    /// Params
    /// ---
    /// - desired_match_id: ID of the match we wish to delete
    ///
    /// Returns
    /// ---
    /// - Ok(GameMatchEvent) if successful to preserve the content of the deleted item
    /// - Err(_) if any error occurrs
    async fn delete(&self, desired_match_id: i32) -> anyhow::Result<GameMatch>;

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
    async fn get_show_info(
        &self,
        desired_match_id: i32,
    ) -> anyhow::Result<(GameMatch, GameMatchEvent)>;

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
    async fn get_all_show_info(
        &self,
        filter_by_time_period: Option<GameMatchEventFilter>,
        filter_by_game: Option<i32>,
    ) -> anyhow::Result<Vec<(GameMatch, GameMatchEvent)>>;

    /// Update a match record.
    /// Update ratios, possibly the supposed start and the shown state of the match
    /// Fails, if the supposed start is less than 2 seconds away
    /// TODO! solve this thing
    ///
    /// Params
    /// ---
    /// - desired_match_id: ID of the match we wish to update the info of
    /// - edited_info: structure containing info that needs to be updated.
    ///
    /// Returns
    /// ---
    /// - Ok(()) if the update was successful
    /// - Err(_) if an error has occurred
    async fn update_info(
        &self,
        desired_match_id: i32,
        edited_info: GameMatchUpdate,
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
    /// - Err(_) if an error occurred or the event we wish to create already exists
    async fn create_event(
        &self,
        desired_match_id: i32,
        desired_event_type: GameMatchEventType,
    ) -> anyhow::Result<i32>;

    /// Get the newest event of the match
    ///
    /// Params
    /// ---
    /// - desired_match_id: ID of the desired match
    ///
    /// Returns
    /// ---
    /// - Ok(newest_event) with the latest event
    /// - Err(_) if there was an internal consistency error / connection error etc
    async fn newest_event(&self, desired_match_id: i32) -> anyhow::Result<GameMatchEvent>;
}

#[async_trait]
impl MatchRepo for PgMatchRepo {
    /// Create a new game match structure (and set the latest event of the match to upcoming)
    /// Additionally, it checks that both teams are playing the chosen game
    async fn create(&self, new_match: CreateGameMatch) -> anyhow::Result<i32> {
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

        let team_one_name: String = team::table
            .find(new_match.team_one_id)
            .select(team::name)
            .get_result(&connection)?;

        let team_two_name: String = team::table
            .find(new_match.team_two_id)
            .select(team::name)
            .get_result(&connection)?;

        let game_name: String = game::table
            .find(new_match.game_id)
            .select(game::name)
            .get_result(&connection)?;

        // create the game match
        let query_result: i32 = insert_into(game_match::table)
            .values(new_match.store(&game_name, &team_one_name, &team_two_name))
            .returning(game_match::id)
            .get_result(&connection)?;

        // create an upcoming event for the new match
        self.create_event(query_result, GameMatchEventType::Upcoming)
            .await?;

        Ok(query_result)
    }

    /// Delete a game match
    /// Only possible if the match is upcoming and there are no bets tied to the match yet.
    /// Deleting the match does not work, if the match is about to start in 5 seconds
    async fn delete(&self, desired_match_id: i32) -> anyhow::Result<GameMatch> {
        let connection: PgPooledConnection = self.get_connection().await?;

        let any_bets: usize = submitted_bet::table
            .filter(bet::game_match_id.eq(desired_match_id))
            .execute(&connection)?;

        // there are bets submitted on the match
        if any_bets > 0 {
            anyhow::bail!("Cannot delete a game match, if there are submitted bets placed on it");
        }

        let to_be_removed: GameMatch = game_match::table
            .find(desired_match_id)
            .get_result(&connection)?;

        // cannot delete an already starting
        if TimeHandling::load_timestamp(&to_be_removed.supposed_start_at)?
            < (Utc::now() + Duration::seconds(2))
        {
            anyhow::bail!("Cannot delete a game match, if it is about to start!");
        }

        // remove all events first
        let _ = delete(
            game_match_event::table.filter(game_match_event::game_match_id.eq(desired_match_id)),
        )
        .execute(&connection)?;

        // remove all unsubmitted bets second
        let _ = delete(bet::table.filter(bet::game_match_id.eq(desired_match_id)))
            .execute(&connection)?;

        // remove the match
        let _ = delete(game_match::table.find(desired_match_id)).execute(&connection)?;

        Ok(to_be_removed)
    }

    /// Get a desired match by its ID
    async fn get(&self, desired_match_id: i32) -> anyhow::Result<GameMatch> {
        let query_result: GameMatch = game_match::table
            .find(desired_match_id)
            .get_result(&self.get_connection().await?)?;

        Ok(query_result)
    }

    /// Get a desired match (together with display information) by its ID
    async fn get_show_info(
        &self,
        desired_match_id: i32,
    ) -> anyhow::Result<(GameMatch, GameMatchEvent)> {
        let query_result: (GameMatch, GameMatchEvent) = game_match::table
            .filter(game_match::id.eq(desired_match_id))
            .inner_join(game_match_event::table)
            .order(game_match_event::created_at.desc())
            .distinct_on(game_match::id)
            .get_result(&self.get_connection().await?)?;

        Ok(query_result)
    }

    /// Get all matches, optionally we can filter by the time period (upcoming),
    /// filter by the team and filter by the game
    async fn get_all_show_info(
        &self,
        filter_by_time_period: Option<GameMatchEventFilter>,
        filter_by_game: Option<i32>,
    ) -> anyhow::Result<Vec<(GameMatch, GameMatchEvent)>> {
        // join the match table with the latest event
        let basic_query = game_match::table
            .inner_join(game_match_event::table)
            .order(game_match_event::created_at.desc())
            .distinct_on(game_match::id)
            .order_by(game_match::supposed_start_at.desc());

        // filter by method parameters
        let query_result: Vec<(GameMatch, GameMatchEvent)> =
            match (filter_by_time_period, filter_by_game) {
                // filter by both period and a game id
                (Some(period), Some(game)) => basic_query
                    .filter(
                        (game_match_event::event_type.eq(period.to_string()))
                            .and(game_match::game_id.eq(game)),
                    )
                    .get_results(&self.get_connection().await?)?,

                // filter by period
                (Some(period), None) => basic_query
                    .filter(game_match_event::event_type.eq(period.to_string()))
                    .get_results(&self.get_connection().await?)?,
                // fiter by game id
                (None, Some(game)) => basic_query
                    .filter(game_match::game_id.eq(game))
                    .get_results(&self.get_connection().await?)?,
                // nofilter
                _ => basic_query.get_results(&self.get_connection().await?)?,
            };

        Ok(query_result)
    }

    /// TODO!
    /// Update a match record.
    /// Update ratios, possibly the supposed start and the shown state of the match
    /// Fails, if the supposed start is less than 2 seconds away
    async fn update_info(
        &self,
        _desired_match_id: i32,
        edited_match: GameMatchUpdate,
    ) -> anyhow::Result<()> {
        // let table = update(game_match::table.find(desired_match_id));

        // let _ = match edited_match.supposed_start_at {
        //     Some(new_start) => table.set(()).execute(&self.get_connection().await?)?,
        //     None => {}
        // };

        todo!()
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

    /// Get newest event of the match
    async fn newest_event(&self, desired_match_id: i32) -> anyhow::Result<GameMatchEvent> {
        let query_result = game_match_event::table
            .filter(game_match_event::game_match_id.eq(desired_match_id))
            .order(game_match_event::created_at.desc())
            .first(&self.get_connection().await?)?;

        Ok(query_result)
    }
}
