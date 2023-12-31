use async_trait::async_trait;
use std::sync::Arc;

use crate::connection::{PgPool, PgPooledConnection};
use crate::diesel::{insert_into, prelude::*, update, QueryDsl, RunQueryDsl};

// type and structure imports
use super::repo::Repo;
use crate::db_models::game::{CreateGame, Game};
use crate::result_types::{GameInfo, TeamInfo};

// schema imports
use crate::schema::{game, team, team_plays_game};

pub struct PgGameRepo {
    pub pool: Arc<PgPool>,
}

#[async_trait]
impl Repo for PgGameRepo {
    /// Create a new Game repo with a reference to an initialized pool
    ///
    /// Params
    /// ---
    /// - pool: A reference to an already initialized database connection pool,
    ///         used for connecting to the database
    ///
    /// Returns
    /// ---
    /// - new Game repo
    fn new(pool: &Arc<PgPool>) -> PgGameRepo {
        PgGameRepo {
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
pub trait GameRepo {
    /// Get details about one specific game
    ///
    /// Params
    /// ---
    /// - desired_game_id: id of the game that's of interest
    ///
    /// Returns
    /// ---
    /// - Ok(game) if the game with the desired id has been found
    /// - Err(_) otherwise
    async fn get(&self, desired_game_id: i32) -> anyhow::Result<Game>;

    /// Edit a certain game
    ///
    /// Params
    /// ---
    /// - desired_game_id: ID of the game we want to update
    /// - edited_game: structure containing updated values for the game
    ///
    /// Returns
    /// ---
    /// - Ok(()) if the update was successful
    /// - Err(_) if an error occurred
    async fn edit(&self, desired_game_id: i32, edited_game: CreateGame) -> anyhow::Result<()>;

    /// Get all game names, id's and image urls
    ///
    /// Returns
    /// ---
    /// - Ok(Vec<(id, name, logo)>) on successful database connection
    /// - Err(_) otherwise
    async fn get_all(&self) -> anyhow::Result<Vec<GameInfo>>;

    /// Get all teams that are playing a specific game
    ///
    /// Params
    /// ---
    /// - desired_game_id: game in which we are interested in
    ///
    /// Returns
    /// ---
    /// - Ok(Vec<TeamInfo>) which contains a vector of team id's, team names and team logo urls
    /// - Err(_) if an error occurrs
    async fn get_teams_playing(&self, desired_game_id: i32) -> anyhow::Result<Vec<TeamInfo>>;

    /// Create a new Game record in the database
    ///
    /// Params
    /// ---
    /// - new_game: structure for database insert of a Game record
    ///
    /// Returns
    /// ---
    /// - Ok(id) with game id after successful creation
    /// - Err(_) if an error occurrs
    async fn create(&self, new_game: CreateGame) -> anyhow::Result<i32>;
}

#[async_trait]
impl GameRepo for PgGameRepo {
    /// Create a new Game record in the database
    async fn create(&self, new_game: CreateGame) -> anyhow::Result<i32> {
        let id: i32 = insert_into(game::table)
            .values(new_game)
            .returning(game::id)
            .get_result(&self.get_connection().await?)?;

        Ok(id)
    }

    /// Edit a certain game specified by id
    async fn edit(&self, desired_game_id: i32, edited_game: CreateGame) -> anyhow::Result<()> {
        let _ = update(game::table.find(desired_game_id))
            .set(edited_game)
            .execute(&self.get_connection().await?)?;

        Ok(())
    }

    /// Get details about one specific game
    async fn get(&self, desired_game_id: i32) -> anyhow::Result<Game> {
        let query_result: Game = game::table
            .find(desired_game_id)
            .get_result(&self.get_connection().await?)?;

        Ok(query_result)
    }

    /// Get all game names and image urls
    async fn get_all(&self) -> anyhow::Result<Vec<GameInfo>> {
        let query_result: Vec<GameInfo> = game::table
            .order(game::name.asc())
            .select((game::id, game::name, game::logo))
            .get_results(&self.get_connection().await?)?;

        Ok(query_result)
    }

    /// Get all teams that are playing a specific game
    async fn get_teams_playing(&self, desired_game_id: i32) -> anyhow::Result<Vec<TeamInfo>> {
        let query_result: Vec<TeamInfo> = game::table
            .find(desired_game_id)
            .inner_join(team_plays_game::table.inner_join(team::table))
            .distinct_on(team::id)
            .select((team::id, team::name, team::logo))
            .get_results(&self.get_connection().await?)?;

        Ok(query_result)
    }
}
