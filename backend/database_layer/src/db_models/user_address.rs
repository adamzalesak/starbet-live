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

#[derive(Insertable, AsChangeset)]
#[table_name = "user_address"]
pub struct CreateUserAddress {
    pub user_id: i32,
    pub street_name: String,
    pub street_number: String,
    pub city: String,
    pub area: Option<String>,
    pub postal_code: String,
    pub country: String,
    pub valid_from: String,
}

impl CreateUserAddress {
    /// Creating the write structure for user address
    ///
    /// Params
    /// ---
    /// - street_name: user's address street name
    /// - street_number: user's address street number
    /// - city: user's address city
    /// - area: OPTIONAL area within the country (ie state, province etc)
    /// - postal_code: user's address postal code
    /// - country: user's address country
    pub fn new(
        street_name: &str,
        street_number: &str,
        city: &str,
        area: Option<String>,
        postal_code: &str,
        country: &str,
    ) -> CreateUserAddress {
        CreateUserAddress {
            user_id: 0,
            street_name: street_name.into(),
            street_number: street_number.into(),
            city: city.into(),
            area: area.into(),
            postal_code: postal_code.into(),
            country: country.into(),
            valid_from: "".into(),
        }
    }

    /// Finish the UserAddress write structure to store it into the database
    ///
    /// Params
    /// ---
    /// - self: the original address created
    /// - user_id: set the ID of the user this address belongs to
    ///
    /// Returns
    /// ---
    /// - a complete address write structure
    pub fn store(&self, user_id: i32) -> CreateUserAddress {
        CreateUserAddress {
            user_id,
            street_name: self.street_name.clone(),
            street_number: self.street_number.clone(),
            city: self.city.clone(),
            area: self.area.clone(),
            postal_code: self.postal_code.clone(),
            country: self.country.clone(),
            valid_from: CurrentTime::store(),
        }
    }
}
