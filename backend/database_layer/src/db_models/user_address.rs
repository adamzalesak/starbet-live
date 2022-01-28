use crate::db_models::user::User;
use crate::schema::user_address;
use crate::type_storing::time_handling::TimeHandling;

/// Read structure, used for data mapping of
/// `user_address` record from the database
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

impl UserAddress {
    /// Create an update structure for `user_address` record
    /// from already loaded address -> efficient when only a small
    /// detail needs to be changed
    ///
    /// Params
    /// ---
    /// - change_street_name: option that allows us to change the street name
    /// - change_street_number: option that allows us to change the street number,
    /// - change_city: option that allows us to change the city,
    /// - change_area: option that allows us add or delete an area,
    /// - change_postal_code: option that allows us to change the postal code,
    /// - change_country: option that allows us to change the country,
    ///
    /// Returns
    /// ---
    /// - new `user_address` update instance
    pub fn edit_address(
        &self,
        change_street_name: Option<&str>,
        change_street_number: Option<&str>,
        change_city: Option<&str>,
        change_area: Option<Option<&str>>,
        change_postal_code: Option<&str>,
        change_country: Option<&str>,
    ) -> CreateUserAddress {
        CreateUserAddress {
            user_id: 0,
            street_name: change_street_name.map_or_else(|| self.street_name.clone(), String::from),
            street_number: change_street_number
                .map_or_else(|| self.street_number.clone(), String::from),
            city: change_city.map_or_else(|| self.city.clone(), String::from),
            area: match change_area {
                Some(value) => value.map(String::from),
                None => self.area.clone(), // original data remains
            },
            postal_code: change_postal_code.map_or_else(|| self.postal_code.clone(), String::from),
            country: change_country.map_or_else(|| self.country.clone(), String::from),
            valid_from: "".into(),
        }
    }
}

/// Write structure, used for inserting
/// `user_address` records into the database
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
    /// Create a new `user_address` insert structure
    ///
    /// Params
    /// ---
    /// - street_name: user's address street name
    /// - street_number: user's address street number
    /// - city: user's address city
    /// - area: OPTIONAL area within the country (ie state, province etc)
    /// - postal_code: user's address postal code
    /// - country: user's address country
    ///
    /// Returns
    /// ---
    /// - new `user_address` insert structure
    pub fn new(
        street_name: &str,
        street_number: &str,
        city: &str,
        area: Option<&str>,
        postal_code: &str,
        country: &str,
    ) -> CreateUserAddress {
        CreateUserAddress {
            user_id: 0,
            street_name: street_name.into(),
            street_number: street_number.into(),
            city: city.into(),
            area: area.map(String::from),
            postal_code: postal_code.into(),
            country: country.into(),
            valid_from: "".into(),
        }
    }

    /// Finish the `user_address` insert structure to store it into the database
    ///
    /// Params
    /// ---
    /// - self: the original address created
    /// - user_id: set the ID of the user this address belongs to
    ///
    /// Returns
    /// ---
    /// - a complete `user_address` insert structure
    pub fn store(&self, user_id: i32) -> CreateUserAddress {
        CreateUserAddress {
            user_id,
            street_name: self.street_name.clone(),
            street_number: self.street_number.clone(),
            city: self.city.clone(),
            area: self.area.clone(),
            postal_code: self.postal_code.clone(),
            country: self.country.clone(),
            valid_from: TimeHandling::store(),
        }
    }
}
