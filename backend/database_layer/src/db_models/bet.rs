use std::fmt::Display;

use crate::db_models::{game_match::GameMatch, team::Team, ticket::Ticket};
use crate::schema::bet;
use crate::type_storing::time_handling::TimeHandling;

/// Structure used for capturing the state of the `bet` record
pub enum BetState {
    Placed,
    Discarded,
    Paid,
    Unsubmitted,
}

/// Read structure, used for data mapping of
/// `bet` record from the database
#[derive(Identifiable, Queryable, Associations, PartialEq)]
#[belongs_to(Ticket)]
#[belongs_to(Team)]
#[belongs_to(GameMatch)]
#[table_name = "bet"]
pub struct Bet {
    pub id: i32,
    pub game_match_id: i32,
    pub ticket_id: i32,
    pub team_id: i32,
    pub bet_ratio: String,
    pub bet_state: String,
    pub created_at: String,
}

/// Write structure, used for inserting
/// `bet` records into the database
#[derive(Insertable)]
#[table_name = "bet"]
pub struct CreateBet {
    pub game_match_id: i32,
    pub ticket_id: i32,
    pub team_id: i32,
    pub bet_ratio: String,
    pub bet_state: String,
    pub created_at: String,
}

impl Bet {
    /// Convert the string representation of the `bet` state into the enum
    ///
    /// Returns
    /// ---
    /// - Ok(BetState) if the record has been stored and retrieved converted correctly
    /// - Err(_) otherwise
    pub fn bet_state(&self) -> anyhow::Result<BetState> {
        match self.bet_state.as_str() {
            "Placed" => Ok(BetState::Placed),
            "Paid" => Ok(BetState::Paid),
            "Discarded" => Ok(BetState::Discarded),
            "Unsubmitted" => Ok(BetState::Unsubmitted),
            _ => anyhow::bail!("Could not convert the bet state from the loaded database record"),
        }
    }
}

impl CreateBet {
    /// Create a new `bet` insert structure
    ///
    /// Params
    /// ---
    /// - game_match_id: ID of the match we place the bet on
    /// - ticket_id: ID of the ticket this bet is put into
    /// - bet_ratio: ratio of the bet
    /// - bet_state: state of the bet (this will be changed later throughout the )
    ///
    /// Returns
    /// ---
    /// - new `bet` insert structure
    pub fn new(
        game_match_id: i32,
        ticket_id: i32,
        team_id: i32,
        bet_ratio: &str,
        bet_state: &str,
    ) -> CreateBet {
        CreateBet {
            game_match_id,
            ticket_id,
            team_id,
            bet_ratio: String::from(bet_ratio),
            bet_state: String::from(bet_state),
            created_at: TimeHandling::store(),
        }
    }
}

impl Display for BetState {
    /// Implement the display trait for converting the enum and writing the result to the database
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = match self {
            BetState::Discarded => "Discarded",
            BetState::Placed => "Placed",
            BetState::Unsubmitted => "Unsubmitted",
            BetState::Paid => "Paid",
        };

        write!(f, "{}", output)
    }
}
