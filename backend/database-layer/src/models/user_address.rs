use crate::schema::user_address;
use crate::models::user::User;

#[derive(Identifiable, Associations, Queryable, PartialEq)]
#[belongs_to(User)]
#[table_name="user_address"]
pub struct UserAddress {
    id: i32,
    user_id: i32,
    street_name: String,
    city: String,
    area: String,
    postal_code: String,
    country: String,
    valid_from: String,
}

// #[derive(Insertable)]
// #[table_name = "user_address"]
// pub struct CreateUserAddress<'a> {
//     user_id: i32,
//     street_name: &'a str,
//     city: &'a str,
//     area: &'a str,
//     postal_code: &'a str,
//     country: &'a str,
//     valid_from: &'a str,
// }
