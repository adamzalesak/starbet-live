use crate::schema::user_address;
use crate::models::user::User;

#[derive(Identifiable, Associations, Queryable, PartialEq)]
#[belongs_to(User)]
#[table_name="user_address"]
pub struct UserAddress {
    pub id: i32,
    pub user_id: i32,
    pub street_name: String,
    pub city: String,
    pub area: String,
    pub postal_code: String,
    pub country: String,
    pub valid_from: String,
}

#[derive(Insertable)]
#[table_name = "user_address"]
pub struct CreateUserAddress<'a> {
    pub user_id: i32,
    pub street_name: &'a str,
    pub city: &'a str,
    pub area: &'a str,
    pub postal_code: &'a str,
    pub country: &'a str,
    pub valid_from: &'a str,
}
