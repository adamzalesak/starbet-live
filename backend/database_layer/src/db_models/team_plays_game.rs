use crate::db_models::{game::Game, team::Team};
use crate::schema::team_plays_game;

/// Read structure, used for data mapping of
/// `team_plays_game` record from the database
#[derive(Identifiable, Associations, Queryable, PartialEq)]
#[belongs_to(Team)]
#[belongs_to(Game)]
#[table_name = "team_plays_game"]
pub struct TeamPlaysGame {
    pub id: i32,
    pub team_id: i32,
    pub game_id: i32,
}

/// Write structure, used for inserting
/// `team_plays_game` records into the database
#[derive(Insertable)]
#[table_name = "team_plays_game"]
pub struct CreateTeamPlaysGame {
    pub team_id: i32,
    pub game_id: i32,
}

impl CreateTeamPlaysGame {
    /// Create a new `team_plays_game` insert structure
    ///
    /// Params
    /// ---
    /// - desired_game_id: ID of the desired game we wish to connect
    /// - desired_team_id: ID of the desired team we wish to connect
    ///
    /// Returns
    /// ---
    /// - new `team_plays_game` insert structure
    pub fn new(desired_game_id: i32, desired_team_id: i32) -> Self {
        Self {
            game_id: desired_game_id,
            team_id: desired_team_id,
        }
    }
}
