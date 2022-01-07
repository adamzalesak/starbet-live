use crate::schema::ticket;
use crate::models::user::User;

#[derive(Identifiable, Associations, Queryable, PartialEq)]
#[belongs_to(User)]
#[table_name = "ticket"]
pub struct Ticket {
    id: i32,
    user_id: i32,
    created_at: String,
    paid_at: Option<String>,
}

#[derive(Insertable)]
#[table_name = "ticket"]
pub struct CreateTicket<'a> {
    user_id: i32,
    created_at: &'a str,
    paid_at: &'a str,
}
