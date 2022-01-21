use async_trait::async_trait;
use diesel::query_builder;
use std::sync::Arc;

use crate::diesel::insert_into;
use crate::diesel::prelude::*;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;

use crate::connection::PgPool;
use crate::connection::PgPooledConnection;

// type and structure imports
use super::repo::Repo;
use crate::db_access::game::GameInfo;
use crate::db_models::{
    game::Game,
    team::{CreateTeam, Team},
    team_plays_game::{CreateTeamPlaysGame, TeamPlaysGame},
};

// schema imports
use crate::schema::{
    game::{
        dsl::{
            description as game_description, id as game_id, logo as game_logo, name as game_name,
        },
        table as game_table,
    },
    team::{
        dsl::{id as team_id, logo as team_logo, name as team_name, team},
        table as team_table,
    },
    team_plays_game::{
        dsl::{game_id as game_id_join, team_id as team_id_join},
        table as team_plays_game_table,
    },
};

/// Structure containing a reference to a database connection pool
/// and methods to access the database
/// to work with Team records
pub struct PgTeamRepo {
    pub pool: Arc<PgPool>,
}

pub type TeamInfo = (i32, String, String);

#[async_trait]
impl Repo for PgTeamRepo {
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
    fn new(pool: Arc<PgPool>) -> PgTeamRepo {
        PgTeamRepo { pool }
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
pub trait TeamRepo {
    async fn get(&self, desired_team_id: i32) -> anyhow::Result<Team>;

    async fn get_all(&self, by_game: Option<i32>) -> anyhow::Result<Vec<TeamInfo>>;

    async fn games_played(&self, desired_team_id: i32) -> anyhow::Result<Vec<GameInfo>>;

    async fn add_to_game(&self, desired_team_id: i32, desired_game_id: i32) -> anyhow::Result<()>;

    async fn create<'a>(&self, new_team: CreateTeam<'a>) -> anyhow::Result<i32>;
}

#[async_trait]
impl TeamRepo for PgTeamRepo {
    /// Get a specific desired team record
    ///
    /// Params
    /// ---
    /// - desired_team_id: ID of the desired team
    ///
    /// Returns
    /// ---
    /// - Ok(team) if the team has been found successfully
    /// - Err(_) if an error occurred
    async fn get(&self, desired_team_id: i32) -> anyhow::Result<Team> {
        let query_result: Team = team
            .find(desired_team_id)
            .get_result(&self.get_connection().await?)?;

        Ok(query_result)
    }

    /// Get all teams (OPT: that play a certain game)
    ///
    /// Params
    /// ---
    /// - by_game_id: if contains a value -> filters which teams to obtain according to which game they play
    ///               if it does not, has no effect on the query and function returns all teams
    ///
    /// Returns
    /// ---
    /// TeamInfo is a type alias for a tuple containing (team_id, team_name, team_logo_url)
    ///
    /// - Ok(Vec<TeamInfo>) if the query has ran successfully (either containing all teams,
    ///                     or just the teams that play a certain game)
    /// - Err(_) if an error has occurred
    async fn get_all(&self, by_game_id: Option<i32>) -> anyhow::Result<Vec<TeamInfo>> {
        let query_result: Vec<TeamInfo> = match by_game_id {
            Some(game_id_filter) => team
                .order(team_name.asc())
                .inner_join(team_plays_game_table)
                .filter(game_id_join.eq(game_id_filter))
                .distinct_on(team_id)
                .select((team_id, team_name, team_logo))
                .get_results(&self.get_connection().await?)?,
            _ => team
                .order(team_name.asc())
                .select((team_id, team_name, team_logo))
                .get_results(&self.get_connection().await?)?,
        };

        Ok(query_result)
    }

    /// Get a list of games which a certain team plays
    ///
    /// Params
    /// ---
    /// - desired_team_id: ID of the desired team
    ///
    /// Returns
    /// ---
    /// GameInfo is a type alias for a touple containing (game_id, game_name, game_logo_url)
    ///
    /// - Ok(Vec<GameInfo>) if the query has been executed successfully
    /// - Err(_) if an error occurred
    async fn games_played(&self, desired_team_id: i32) -> anyhow::Result<Vec<GameInfo>> {
        let query_result: Vec<GameInfo> = team
            .inner_join(team_plays_game_table.inner_join(game_table))
            .filter(team_id.eq(desired_team_id))
            .select((game_id, game_name, game_logo))
            .distinct_on(game_id)
            .get_results(&self.get_connection().await?)?;

        Ok(query_result)
    }

    async fn add_to_game(&self, desired_team_id: i32, desired_game_id: i32) -> anyhow::Result<()> {
        todo!()
    }

    async fn create<'a>(&self, new_team: CreateTeam<'a>) -> anyhow::Result<i32> {
        todo!()
    }
}
