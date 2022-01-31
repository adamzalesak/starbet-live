use chrono::{DateTime, Utc};

use crate::db_models::game::Game;
use crate::schema::game_match;

use diesel::sql_types::Double;

/// Read structure, used for data mapping of
/// `game_match` record from the database
#[derive(Identifiable, Associations, Queryable, QueryableByName, PartialEq)]
#[belongs_to(Team, foreign_key = team_one_id)]
#[belongs_to(Team, foreign_key = team_two_id)]
#[belongs_to(Game)]
#[table_name = "game_match"]
pub struct GameMatch {
    pub id: i32,
    pub game_id: i32,
    pub game_name: String,
    pub team_one_id: i32,
    pub team_one_name: String,
    pub team_one_ratio: String,
    pub team_two_id: i32,
    pub team_two_ratio: String,
    pub team_two_name: String,
    pub supposed_start_at: String,
    pub state: String,
}

/// Write structure, used for inserting
/// `game_match` records into the database
#[derive(Insertable)]
#[table_name = "game_match"]
/// Update the state for chosen match
pub struct CreateGameMatch {
    pub game_id: i32,
    pub game_name: String,
    pub team_one_id: i32,
    pub team_one_name: String,
    pub team_one_ratio: String,
    pub team_two_id: i32,
    pub team_two_ratio: String,
    pub team_two_name: String,
    pub supposed_start_at: String,
    pub state: String,
}

impl GameMatch {
    /// Create an update structure for `game_match` record
    ///
    /// Params
    /// ---
    /// - update_ratio_one: option to change the first ratio
    /// - update_ratio_two: option to change the second ratio,
    /// - update_supposed_start_at: option to change the supposed start of the match
    ///                             (note -> the query will fail if the supposed start is in the past, or the match has already started)
    /// - update_state: option to update the state of the match
    ///
    /// Returns
    /// ---
    /// - new `game_match` update structure
    pub fn to_update(
        &self,
        update_ratio_one: Option<&str>,
        update_ratio_two: Option<&str>,
        update_state: Option<&str>,
    ) -> anyhow::Result<GameMatchUpdate> {
        Ok(GameMatchUpdate {
            team_one_ratio: update_ratio_one
                .map_or_else(|| self.team_one_ratio.clone(), String::from),
            team_two_ratio: update_ratio_two
                .map_or_else(|| self.team_two_ratio.clone(), String::from),
            state: update_state.map_or_else(|| self.state.clone(), String::from),
        })
    }
}

impl CreateGameMatch {
    /// Create a new `game_match` insert structure
    ///
    /// Params
    /// ---
    /// - game_id: ID of the game the match is of
    /// - team_one_id: ID of the first team
    /// - team_two_id: ID of the second team
    /// - team_one_ratio: bet ratio of the first team
    /// - team_two_ratio: bet ratio of the second team
    /// - supposed_start_at: when the match is supposed to start
    /// - state: display string
    ///
    /// Returns
    /// ---
    /// - new match write structure
    pub fn new(
        game_id: i32,
        team_one_id: i32,
        team_two_id: i32,
        team_one_ratio: &str,
        team_two_ratio: &str,
        supposed_start_at: DateTime<Utc>,
        state: &str,
    ) -> CreateGameMatch {
        CreateGameMatch {
            game_id,
            game_name: String::from(""),
            team_one_id,
            team_one_ratio: String::from(team_one_ratio),
            team_one_name: String::from(""),
            team_two_id,
            team_two_ratio: String::from(team_two_ratio),
            team_two_name: String::from(""),
            supposed_start_at: supposed_start_at.to_string(),
            state: String::from(state),
        }
    }

    /// Create a stored record, when retrieving necessary info from the database.
    pub fn store(
        &self,
        game_name: &str,
        team_one_name: &str,
        team_two_name: &str,
    ) -> CreateGameMatch {
        CreateGameMatch {
            game_id: self.game_id,
            game_name: String::from(game_name),
            team_one_id: self.team_one_id,
            team_one_ratio: self.team_one_ratio.clone(),
            team_one_name: String::from(team_one_name),
            team_two_id: self.team_two_id,
            team_two_ratio: self.team_two_ratio.clone(),
            team_two_name: String::from(team_two_name),
            supposed_start_at: self.supposed_start_at.clone(),
            state: self.state.clone(),
        }
    }
}

/// Structure which allows us to edit editable game match properties
pub struct GameMatchUpdate {
    pub team_one_ratio: String,
    pub team_two_ratio: String,
    pub state: String,
}
