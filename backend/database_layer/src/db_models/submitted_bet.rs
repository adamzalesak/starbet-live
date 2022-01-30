use crate::db_models::{game_match::GameMatch, submitted_ticket::SubmittedTicket, team::Team};
use crate::schema::submitted_bet;
use crate::type_storing::time_handling::TimeHandling;

/// Read structure, used for data mapping of
/// `bet` record from the database
#[derive(Identifiable, Queryable, Associations, PartialEq)]
#[belongs_to(SubmittedTicket)]
#[belongs_to(Team)]
#[belongs_to(GameMatch)]
#[table_name = "submitted_bet"]
pub struct SubmittedBet {
    pub id: i32,
    pub game_match_id: i32,
    pub submitted_ticket_id: i32,
    pub team_id: i32,
    pub bet_ratio: String,
    pub placed_at: String,
    pub submitted_at: String,
    won: Option<bool>,
}

/// Write structure, used for inserting
/// `bet` records into the database
#[derive(Insertable)]
#[table_name = "submitted_bet"]
pub struct CreateSubmittedBet {
    pub game_match_id: i32,
    pub submitted_ticket_id: i32,
    pub team_id: i32,
    pub bet_ratio: String,
    pub placed_at: String,
    pub submitted_at: String,
    won: Option<bool>,
}

impl CreateSubmittedBet {
    /// Create a new `bet` insert structure
    ///
    /// Params
    /// ---
    /// - game_match_id: ID of the match we place the bet on
    /// - ticket_id: ID of the ticket this bet is put into
    /// - bet_ratio: ratio of the bet
    ///
    /// Returns
    /// ---
    /// - new `bet` insert structure
    pub fn new(
        game_match_id: i32,
        submitted_ticket_id: i32,
        team_id: i32,
        bet_ratio: &str,
        placed_at: &str,
        submitted_at: &str,
    ) -> CreateSubmittedBet {
        CreateSubmittedBet {
            game_match_id,
            submitted_ticket_id,
            team_id,
            bet_ratio: String::from(bet_ratio),
            placed_at: String::from(placed_at),
            submitted_at: TimeHandling::store(),
            won: None,
        }
    }
}
