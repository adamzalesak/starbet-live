use crate::db_models::user::User;
use crate::schema::user_address;
use crate::type_storing::time_handling::CurrentTime;

#[derive(Identifiable, Associations, Queryable, PartialEq)]
#[belongs_to(User)]
#[table_name = "user_address"]
pub struct UserAddress {
    pub id: i32,
    pub user_id: i32,
    pub street_name: String,
    pub street_number: String,
    pub city: String,
    pub area: Option<String>,
    pub postal_code: String,
    pub country: String,
    pub valid_from: String,
}

#[derive(Insertable)]
#[table_name = "user_address"]
pub struct CreateUserAddress<'a> {
    pub user_id: i32,
    pub street_name: &'a str,
    pub street_number: &'a str,
    pub city: &'a str,
    pub area: Option<&'a str>,
    pub postal_code: &'a str,
    pub country: &'a str,
    pub valid_from: String,
}

impl<'a> CreateUserAddress<'a> {
    /// Creating the write structure for user address
    ///
    /// Params
    /// ---
    /// - user_id: desired user to link the address to
    /// - street_name: user's address street name
    /// - street_number: user's address street number
    /// - city: user's address city
    /// - area: OPTIONAL area within the country (ie state, province etc)
    /// - postal_code: user's address postal code
    /// - country: user's address country
    pub fn new(
        user_id: i32,
        street_name: &'a str,
        street_number: &'a str,
        city: &'a str,
        area: Option<&'a str>,
        postal_code: &'a str,
        country: &'a str,
    ) -> CreateUserAddress<'a> {
        CreateUserAddress {
            user_id,
            street_name,
            street_number,
            city,
            area,
            postal_code,
            country,
            valid_from: CurrentTime::store(),
        }
    }
}
