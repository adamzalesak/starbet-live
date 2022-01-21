use async_trait::async_trait;
use std::any;
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
    team::{CreateTeam, Team},
    team_plays_game::{CreateTeamPlaysGame, TeamPlaysGame},
};

/// Structure containing possibilities of ordering matches
pub enum MatchOrderBy {
    GameName,
    LivesFirst,
}

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
    async fn get(&self, desired_match_id: i32) -> anyhow::Result<GameMatch>;

    async fn get_all(
        &self,
        filter_by_time_period: Option<GameMatchEventType>,
        order_by: Option<MatchOrderBy>,
        filter_by_team: Option<i32>,
    ) -> anyhow::Result<Vec<Team>>;

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
    async fn get(&self, desired_match_id: i32) -> anyhow::Result<GameMatch> {
        todo!()
    }

    async fn get_all(
        &self,
        filter_by_time_period: Option<GameMatchEventType>,
        order_by: Option<MatchOrderBy>,
        filter_by_team: Option<i32>,
    ) -> anyhow::Result<Vec<Team>> {
        todo!()
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
