use async_trait::async_trait;
use std::sync::Arc;

use crate::diesel::prelude::*;
use crate::diesel::QueryDsl;
use crate::diesel::RunQueryDsl;
use crate::diesel::{delete, insert_into, update};

use crate::connection::PgPool;
use crate::connection::PgPooledConnection;

// type and structure imports
use super::repo::Repo;
use crate::db_models::{
    team::{CreateTeam, Team},
    team_plays_game::CreateTeamPlaysGame,
};
use crate::result_types::{GameInfo, GameInfoRetrieve, TeamInfo, TeamInfoRetrieve};

// schema imports
use crate::schema::{
    game::{
        dsl::{id as game_id, logo as game_logo, name as game_name},
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
    fn new(pool: &Arc<PgPool>) -> PgTeamRepo {
        PgTeamRepo {
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

impl PgTeamRepo {
    /// Check if the team belongs to the game or not
    ///
    /// Params
    /// ---
    /// - desired_team_id: ID of the desired team
    /// - desired_game_id: ID of the game we want to check the team belonging to
    ///
    /// Returns
    /// ---
    /// - Ok(number) - number of records (if done correctly only getting a 1 or 0)
    /// - Err(_) if an error occurred
    async fn in_game(&self, desired_team_id: i32, desired_game_id: i32) -> anyhow::Result<usize> {
        let number_of_rows: usize = team_plays_game_table
            .filter(team_id_join.eq(desired_team_id))
            .filter(game_id_join.eq(desired_game_id))
            .execute(&self.get_connection().await?)?;

        Ok(number_of_rows)
    }
}

#[async_trait]
pub trait TeamRepo {
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
    async fn get(&self, desired_team_id: i32) -> anyhow::Result<Team>;

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
    async fn get_all(&self, by_game: Option<i32>) -> anyhow::Result<Vec<TeamInfo>>;

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
    async fn games_played(&self, desired_team_id: i32) -> anyhow::Result<Vec<GameInfo>>;

    /// Add a team into a list of teams that play the game
    /// It is only possible, if the team is not already in the
    ///
    /// Params
    /// ---
    /// - desired_team_id: ID of the team we want to add
    /// - desired_game_id: ID of the game we want to add the team into
    ///
    /// Returns
    /// ---
    /// - Ok(()) if the addition has been successful
    /// - Err(_) if an error occurred while creating
    async fn add_to_game(&self, desired_team_id: i32, desired_game_id: i32) -> anyhow::Result<()>;

    /// Remove a team from playing a certain game
    ///
    /// Params
    /// ---
    /// - desired_team_id: ID of the team we want to add
    /// - desired_game_id: ID of the game we want to add the team into
    ///
    /// Returns
    /// ---
    /// - Ok(()) if the deletion has been successful
    /// - Err(_) if an error occurred while deleting
    async fn remove_from_game(
        &self,
        desired_team_id: i32,
        desired_game_id: i32,
    ) -> anyhow::Result<()>;

    /// Edit the team information
    ///
    /// Params
    /// ---
    /// - desired_team_id: ID of the desired team
    /// - edited_team: a write structure with edited properties
    ///
    /// Returns
    /// ---
    /// - Ok(()) if the team information has updated successfully
    /// - Err(_) if an error occurred
    async fn edit<'a>(
        &self,
        desired_team_id: i32,
        edited_team: CreateTeam<'a>,
    ) -> anyhow::Result<()>;

    /// Create a new team
    ///
    /// Params
    /// ---
    /// - new_team: a write structure for creating a team
    ///
    /// Returns
    /// ---
    /// - Ok(id) if the new team has been created successfully
    /// - Err(_) if an error occurred
    async fn create<'a>(&self, new_team: CreateTeam<'a>) -> anyhow::Result<i32>;
}

#[async_trait]
impl TeamRepo for PgTeamRepo {
    /// Get a specific desired team record
    async fn get(&self, desired_team_id: i32) -> anyhow::Result<Team> {
        let query_result: Team = team
            .find(desired_team_id)
            .get_result(&self.get_connection().await?)?;

        Ok(query_result)
    }

    /// Get all teams (OPT: that play a certain game)
    async fn get_all(&self, by_game_id: Option<i32>) -> anyhow::Result<Vec<TeamInfo>> {
        let query_result: Vec<TeamInfoRetrieve> = match by_game_id {
            Some(game_id_filter) => team
                .order(team_name.asc())
                .inner_join(team_plays_game_table)
                .filter(game_id_join.eq(game_id_filter))
                .select((team_id, team_name, team_logo))
                .get_results(&self.get_connection().await?)?,
            _ => team
                .order(team_name.asc())
                .select((team_id, team_name, team_logo))
                .get_results(&self.get_connection().await?)?,
        };

        Ok(TeamInfo::from_vector(&query_result))
    }

    /// Get a list of games which a certain team plays
    async fn games_played(&self, desired_team_id: i32) -> anyhow::Result<Vec<GameInfo>> {
        let query_result: Vec<GameInfoRetrieve> = team
            .inner_join(team_plays_game_table.inner_join(game_table))
            .filter(team_id.eq(desired_team_id))
            .select((game_id, game_name, game_logo))
            .distinct_on(game_id)
            .get_results(&self.get_connection().await?)?;

        Ok(GameInfo::from_vector(&query_result))
    }

    /// Add a team into a list of teams that play the game
    async fn add_to_game(&self, desired_team_id: i32, desired_game_id: i32) -> anyhow::Result<()> {
        // check whether the team is already playing the game, if it is, return an error
        if self.in_game(desired_team_id, desired_game_id).await? != 0 {
            anyhow::bail!("The team already plays the game!");
        }

        // add the team to the game
        let _: usize = insert_into(team_plays_game_table)
            .values(CreateTeamPlaysGame::new(desired_game_id, desired_team_id))
            .execute(&self.get_connection().await?)?;

        // all went well
        Ok(())
    }

    /// Remove a team from playing a certain game
    async fn remove_from_game(
        &self,
        desired_team_id: i32,
        desired_game_id: i32,
    ) -> anyhow::Result<()> {
        // check whether the team is playing the game, if it isnt, return an error
        if self.in_game(desired_team_id, desired_game_id).await? == 0 {
            anyhow::bail!("The team does not play the game!");
        }

        // remove the team from the game
        let _: usize = delete(
            team_plays_game_table
                .filter(team_id_join.eq(desired_team_id))
                .filter(game_id_join.eq(desired_game_id)),
        )
        .execute(&self.get_connection().await?)?;

        // all went well
        Ok(())
    }

    /// Edit the team information
    async fn edit<'a>(
        &self,
        desired_team_id: i32,
        edited_team: CreateTeam<'a>,
    ) -> anyhow::Result<()> {
        let _ = update(team_table.find(desired_team_id))
            .set(edited_team)
            .execute(&self.get_connection().await?)?;

        Ok(())
    }

    /// Create a new team
    async fn create<'a>(&self, new_team: CreateTeam<'a>) -> anyhow::Result<i32> {
        let query_result: i32 = insert_into(team_table)
            .values(new_team)
            .returning(team_id)
            .get_result(&self.get_connection().await?)?;

        Ok(query_result)
    }
}
