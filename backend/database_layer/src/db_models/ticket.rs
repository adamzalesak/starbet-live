use crate::db_models::user::User;
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
    pub price: String,
}

/// Write structure, used for inserting
/// `ticket` records into the database
#[derive(Insertable)]
#[table_name = "ticket"]
pub struct CreateTicket {
    pub user_id: i32,
    pub created_at: String,
    pub valid_until: String,
    pub price: String,
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
    pub fn new(user_id: i32, price: &str) -> CreateTicket {
        CreateTicket {
            user_id,
            created_at: TimeHandling::store(),
            valid_until: (Utc::now() + Duration::days(10)).to_string(),
            price: String::from(price),
        }
    }
}
