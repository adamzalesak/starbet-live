use crate::models::user::User;
use crate::schema::ticket;

#[derive(Identifiable, Associations, Queryable, PartialEq)]
#[belongs_to(User)]
#[table_name = "ticket"]
pub struct Ticket {
    pub id: i32,
    pub user_id: i32,
    pub created_at: String,
    pub paid_at: Option<String>,
}

#[derive(Insertable)]
#[table_name = "ticket"]
pub struct CreateTicket<'a> {
    pub user_id: i32,
    pub created_at: &'a str,
    pub paid_at: Option<&'a str>,
}
