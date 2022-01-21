use async_trait::async_trait;
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
    game::{CreateGame, Game},
    team::Team,
};

// schema imports
use crate::schema::{
    game::{
        dsl::{game, id as game_id, logo as game_logo, name as game_name},
        table as game_table,
    },
    team::{
        dsl::{id as team_id, team},
        table as team_table,
    },
    team_plays_game::{
        dsl::{
            game_id as join_game_id, id as team_plays_game_id, team_id as join_team_id,
            team_plays_game,
        },
        table as team_plays_game_table,
    },
};

pub struct PgGameRepo {
    pub pool: Arc<PgPool>,
}

pub type GameInfo = (i32, String, String);

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
    fn new(pool: Arc<PgPool>) -> PgGameRepo {
        PgGameRepo { pool }
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
    async fn get(&self, desired_game_id: i32) -> anyhow::Result<Game>;

    async fn get_all(&self) -> anyhow::Result<Vec<GameInfo>>;

    async fn get_teams_playing(&self, desired_game_id: i32) -> anyhow::Result<Vec<Team>>;

    async fn create<'a>(&self, new_game: CreateGame<'a>) -> anyhow::Result<i32>;
}

#[async_trait]
impl GameRepo for PgGameRepo {
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
    async fn get(&self, desired_game_id: i32) -> anyhow::Result<Game> {
        let query_result: Game = game
            .find(desired_game_id)
            .get_result(&self.get_connection().await?)?;

        Ok(query_result)
    }

    /// Get all game names and image urls
    ///
    /// Returns
    /// ---
    /// - Ok(Vec<(id, name, logo)>) on successful database connection
    /// - Err(_) otherwise
    async fn get_all(&self) -> anyhow::Result<Vec<GameInfo>> {
        let query_result: Vec<GameInfo> = game
            .order(game_name.asc())
            .select((game_id, game_name, game_logo))
            .get_results(&self.get_connection().await?)?;

        Ok(query_result)
    }

    /// Get all teams that are playing a specific game
    ///
    /// Params
    /// ---
    /// - desired_game_id: game in which we are interested in
    ///
    ///
    async fn get_teams_playing(&self, desired_game_id: i32) -> anyhow::Result<Vec<Team>> {
        todo!()
    }

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
    async fn create<'a>(&self, new_game: CreateGame<'a>) -> anyhow::Result<i32> {
        let id: Result<i32, _> = insert_into(game_table)
            .values(new_game)
            .returning(game_id)
            .get_result(&self.get_connection().await?);

        Ok(id?)
    }
}
