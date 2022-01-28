use crate::db_models::user::User;
use crate::{schema::ticket, type_storing::time_handling::TimeHandling};

/// Read structure, used for data mapping of
/// `ticket` record from the database
#[derive(Identifiable, Associations, Queryable, PartialEq, Clone)]
#[belongs_to(User)]
#[table_name = "ticket"]
pub struct Ticket {
    pub id: i32,
    pub user_id: i32,
    pub created_at: String,
    pub price: String,
    pub paid_at: Option<String>,
}

/// Write structure, used for inserting
/// `ticket` records into the database
#[derive(Insertable)]
#[table_name = "ticket"]
pub struct CreateTicket {
    pub user_id: i32,
    pub created_at: String,
    pub price: String,
    pub paid_at: Option<String>,
}

impl CreateTicket {
    /// Create a new `ticket` insert structure
    ///
    /// Params
    /// ---
    /// - user_id: ID of the user we wish to link the ticket to
    /// - price: how much is the user going to pay for this ticket
    ///  
    /// Returns
    /// ---
    /// - new `ticket` insert structure
    pub fn new(user_id: i32, price: String) -> CreateTicket {
        CreateTicket {
            user_id,
            created_at: TimeHandling::store(),
            price,
            paid_at: None,
        }
    }
}
