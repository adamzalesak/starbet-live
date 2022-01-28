use async_trait::async_trait;
use std::sync::Arc;

use crate::connection::{PgPool, PgPooledConnection};
use crate::diesel::{delete, insert_into, prelude::*, update, QueryDsl, RunQueryDsl};
use std::cmp::Ordering;

// type and structure imports
use super::repo::Repo;
use crate::db_models::{
    team::{CreateTeam, Team},
    team_plays_game::CreateTeamPlaysGame,
};
use crate::result_types::{GameInfo, TeamInfo};

// schema imports
use crate::schema::{game, team, team_plays_game};

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
        let number_of_rows: usize = team_plays_game::table
            .filter(team_plays_game::team_id.eq(desired_team_id))
            .filter(team_plays_game::game_id.eq(desired_game_id))
            .execute(&self.get_connection().await?)?;

        Ok(number_of_rows)
    }
}

#[async_trait]
pub trait TeamRepo {
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
    async fn create(&self, new_team: CreateTeam) -> anyhow::Result<i32>;

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
    async fn edit(&self, desired_team_id: i32, edited_team: CreateTeam) -> anyhow::Result<()>;
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

    /// Get all teams
    ///
    /// Returns
    /// ---
    /// TeamInfo is a type alias for a tuple containing (team_id, team_name, team_logo_url)
    ///
    /// - Ok(Vec<TeamInfo>) with all teams on the site
    /// - Err(_) if an error has occurred
    async fn get_all(&self) -> anyhow::Result<Vec<TeamInfo>>;

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
}

#[async_trait]
impl TeamRepo for PgTeamRepo {
    /// Create a new team
    async fn create(&self, new_team: CreateTeam) -> anyhow::Result<i32> {
        let query_result: i32 = insert_into(team::table)
            .values(new_team)
            .returning(team::id)
            .get_result(&self.get_connection().await?)?;

        Ok(query_result)
    }

    /// Edit the team information
    async fn edit(&self, desired_team_id: i32, edited_team: CreateTeam) -> anyhow::Result<()> {
        let _ = update(team::table.find(desired_team_id))
            .set(edited_team)
            .execute(&self.get_connection().await?)?;

        Ok(())
    }

    /// Get a specific desired team record
    async fn get(&self, desired_team_id: i32) -> anyhow::Result<Team> {
        let query_result: Team = team::table
            .find(desired_team_id)
            .get_result(&self.get_connection().await?)?;

        Ok(query_result)
    }

    /// Get all teams
    async fn get_all(&self) -> anyhow::Result<Vec<TeamInfo>> {
        let selection = (team::id, team::name, team::logo);

        let query_result: Vec<TeamInfo> = team::table
            .order(team::name.asc())
            .select(selection)
            .get_results(&self.get_connection().await?)?;

        Ok(query_result)
    }

    /// Get a list of games which a certain team plays
    async fn games_played(&self, desired_team_id: i32) -> anyhow::Result<Vec<GameInfo>> {
        let query_result: Vec<GameInfo> = team::table
            .inner_join(team_plays_game::table.inner_join(game::table))
            .filter(team::id.eq(desired_team_id))
            .select((game::id, game::name, game::logo))
            .distinct_on(game::id)
            .get_results(&self.get_connection().await?)?;

        Ok(query_result)
    }

    /// Add a team into a list of teams that play the game
    async fn add_to_game(&self, desired_team_id: i32, desired_game_id: i32) -> anyhow::Result<()> {
        // check whether the team is already playing the game, if it is, return an error
        let in_game = self.in_game(desired_team_id, desired_game_id).await?;

        match in_game.cmp(&1_usize) {
            Ordering::Less => {},
            Ordering::Equal => anyhow::bail!("The team already plays the game!"),
            Ordering::Greater => anyhow::bail!("The team already plays a game multiple times! This is a bug, please contact the site administrator."),
        };

        // add the team to the game
        let _ = insert_into(team_plays_game::table)
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
        let in_game = self.in_game(desired_team_id, desired_game_id).await?;
        if in_game == 0 {
            anyhow::bail!("The team does not play the game!");
        } else if in_game > 1 {
            anyhow::bail!("Internal error, the team is registered to play the game more than once. Contact the site administrator, this is a bug.");
        }

        // remove the team from the game
        let _ = delete(
            team_plays_game::table.filter(
                team_plays_game::team_id
                    .eq(desired_team_id)
                    .and(team_plays_game::game_id.eq(desired_game_id)),
            ),
        )
        .execute(&self.get_connection().await?)?;

        // all went well
        Ok(())
    }
}
