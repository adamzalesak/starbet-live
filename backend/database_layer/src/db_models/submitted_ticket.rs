use crate::db_models::user::User;
use crate::{schema::submitted_ticket, type_storing::time_handling::TimeHandling};
use chrono::{Duration, Utc};

use std::hash::{Hash, Hasher};

#[derive(Identifiable, Associations, Queryable, PartialEq, Eq, Clone)]
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
