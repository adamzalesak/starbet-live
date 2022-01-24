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

impl UserAddress {
    /// Create a new CreateUserAddress record which can speed up the process if a
    /// user just tried to change a small detail
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
    /// - new edited CreateUserAddress instance
    pub fn edit_address(
        &self,
        change_street_name: Option<&str>,
        change_street_number: Option<&str>,
        change_city: Option<&str>,
        change_area: Option<Option<&str>>,
        change_postal_code: Option<&str>,
        change_country: Option<&str>,
    ) -> CreateUserAddress {
        // create the needed type
        let store_area = match change_area {
            Some(value) => value.map(|content| String::from(content)),
            None => self.area.clone(), // original data remains
        };

        CreateUserAddress {
            user_id: 0,
            street_name: UserAddress::store_change(&self.street_name, &change_street_name),
            street_number: UserAddress::store_change(&self.street_number, &change_street_number),
            city: UserAddress::store_change(&self.city, &change_city),
            area: store_area,
            postal_code: UserAddress::store_change(&self.postal_code, &change_postal_code),
            country: UserAddress::store_change(&self.country, &change_country),
            valid_from: "".into(),
        }
    }

    /// Store a change -> either just copy the original parameter,
    /// or get a new one
    ///
    /// Params
    /// ---
    /// - original_parameter: reference to the original parameter
    /// - new_parameter: optional new parameter -> takes the place of the original parameter
    ///
    /// Returns
    /// ---
    /// - either the old or the new parameter
    fn store_change(original_parameter: &str, new_parameter: &Option<&str>) -> String {
        match new_parameter {
            Some(change_parameter) => String::from(*change_parameter),
            None => String::from(original_parameter),
        }
    }
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
        area: Option<&str>,
        postal_code: &str,
        country: &str,
    ) -> CreateUserAddress {
        CreateUserAddress {
            user_id: 0,
            street_name: street_name.into(),
            street_number: street_number.into(),
            city: city.into(),
            area: area.map(|content| String::from(content)),
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
