use std::fmt::Display;

use crate::db_models::{
    game_match::GameMatch, submitted_bet::CreateSubmittedBet, team::Team, ticket::Ticket,
};
use crate::schema::bet;
use crate::type_storing::time_handling::TimeHandling;

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
    pub created_at: String,
}

impl Bet {
    pub fn submit_bets(desired_submitted_ticket_id: i32, bets: &[Bet]) -> Vec<CreateSubmittedBet> {
        let mut submitted_bets: Vec<CreateSubmittedBet> = Vec::new();
        let submission_date = TimeHandling::store();

        for bet in bets {
            submitted_bets.push(CreateSubmittedBet {
                game_match_id: bet.game_match_id,
                submitted_ticket_id: desired_submitted_ticket_id,
                team_id: bet.team_id,
                bet_ratio: bet.bet_ratio.clone(),
                placed_at: bet.created_at.clone(),
                submitted_at: submission_date.clone(),
                won: None,
            })
        }

        submitted_bets
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
    ///
    /// Returns
    /// ---
    /// - new `bet` insert structure
    pub fn new(game_match_id: i32, ticket_id: i32, team_id: i32, bet_ratio: &str) -> CreateBet {
        CreateBet {
            game_match_id,
            ticket_id,
            team_id,
            bet_ratio: String::from(bet_ratio),
            created_at: TimeHandling::store(),
        }
    }
}
