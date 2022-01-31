use crate::db_models::{bet::Bet, submitted_ticket::CreateSubmittedTicket, user::User};
use crate::{schema::ticket, type_storing::time_handling::TimeHandling};
use chrono::{Duration, Utc};

/// encapuslates an obtained ticket
pub enum ObtainedTicket {
    NewAfterInvalid(Ticket),
    NoTicketFound(Ticket),
    StillValid(Ticket),
}

/// Read structure, used for data mapping of
/// `ticket` record from the database
#[derive(Identifiable, Associations, Queryable, PartialEq, Clone)]
#[belongs_to(User)]
#[table_name = "ticket"]
pub struct Ticket {
    pub id: i32,
    pub user_id: i32,
    pub created_at: String,
    pub valid_until: String,
}

/// Write structure, used for inserting
/// `ticket` records into the database
#[derive(Insertable)]
#[table_name = "ticket"]
pub struct CreateTicket {
    pub user_id: i32,
    pub created_at: String,
    pub valid_until: String,
}

impl Ticket {
    pub fn submit(&self, paid_price: f64, bets: &[Bet]) -> anyhow::Result<CreateSubmittedTicket> {
        if self.valid_until <= Utc::now().to_string() {
            anyhow::bail!("Cannot send an invalid ticket!")
        } else if bets.len() == 0 {
            anyhow::bail!("Cannot submit an empty ticket!")
        }

        let ratio: Option<f64> = bets
            .iter()
            .map(|bet| bet.bet_ratio.parse::<f64>().ok())
            .filter_map(|parsed_ratio| parsed_ratio)
            .reduce(|element_one, element_two| element_one * element_two);

        if ratio.is_none() {
            anyhow::bail!("Could not compute the ratio for the final bet!")
        }

        // ratio will always get obtained via unwrap
        let winnable_price = ratio.unwrap_or(1.0) * paid_price;

        // create the new submitted ticket
        Ok(CreateSubmittedTicket {
            user_id: self.user_id,
            submitted_at: TimeHandling::store(),
            price_paid: paid_price.to_string(),
            winnable_price: winnable_price.to_string(),
            total_ratio: ratio.unwrap_or(1.0).to_string(),
            won: None,
        })
    }
}

impl CreateTicket {
    /// Create a new `ticket` insert structure
    /// The ticket is valid for 10 days. This changes, when the ticket has a bet in it.
    /// The ticket is then valid until the first match that user put a bet ends
    ///
    /// Params
    /// ---
    /// - user_id: ID of the user we wish to link the ticket to
    /// - price: how much is the user going to pay for this ticket
    ///  
    /// Returns
    /// ---
    /// - new `ticket` insert structure
    pub fn new(user_id: i32) -> CreateTicket {
        CreateTicket {
            user_id,
            created_at: TimeHandling::store(),
            valid_until: (Utc::now() + Duration::days(10)).to_string(),
        }
    }
}
