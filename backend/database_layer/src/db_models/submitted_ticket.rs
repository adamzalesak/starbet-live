use crate::db_models::user::User;
use crate::schema::submitted_ticket;

use std::hash::{Hash, Hasher};

#[derive(Identifiable, Associations, Queryable, Clone)]
#[belongs_to(User)]
#[table_name = "submitted_ticket"]
pub struct SubmittedTicket {
    pub id: i32,
    pub user_id: i32,
    pub submitted_at: String,
    pub price_paid: String,
    pub total_ratio: String,
    pub winnable_price: String,
    pub won: Option<bool>,
}

/// needed for sorting this structure efficiently
impl Hash for SubmittedTicket {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.id.hash(state);
    }
}

impl PartialEq for SubmittedTicket {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for SubmittedTicket {}

#[derive(Insertable)]
#[table_name = "submitted_ticket"]
pub struct CreateSubmittedTicket {
    pub user_id: i32,
    pub submitted_at: String,
    pub price_paid: String,
    pub total_ratio: String,
    pub winnable_price: String,
    pub won: Option<bool>,
}
