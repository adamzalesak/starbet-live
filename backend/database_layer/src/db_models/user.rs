use crate::schema::user;
use crate::type_storing::time_handling::CurrentTime;
use chrono::{DateTime, Utc};

/// Read structure, used for data mapping of
/// User record from the database
#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub civil_id_number: String,
    pub email: String,
    pub date_of_birth: String,
    pub phone_number: String,
    pub created_at: String,
    pub photo: Option<String>,
}

/// Write structure, used for inserting
/// User records into the database
#[derive(Insertable, AsChangeset)]
#[table_name = "user"]
pub struct CreateUser {
    pub first_name: String,
    pub last_name: String,
    pub civil_id_number: String,
    pub email: String,
    pub date_of_birth: String,
    pub phone_number: String,
    pub created_at: String,
    pub photo: Option<String>,
}

impl User {
    /// Create a new edit record where each parameter of the user structure apart from the `id`
    /// and `created_at` can be modified
    ///
    /// Params
    /// ---
    /// change_* -> if None present, the original value of the record remains,
    ///             if Some(value) present -> the new value is used
    ///
    /// - change_first_name: ability to change the first name
    /// - change_last_name: ability to change the last name
    /// - change_civil_id_number: ability to change the cvil id number
    /// - change_email: ability to change the email address
    /// - change_date_of_birth: ability to change the date of birth
    /// - change_phone_number: ability to change the phone number
    /// - change_photo: ability to change the profile photo
    ///
    pub fn edit_user(
        &self,
        change_first_name: Option<&str>,
        change_last_name: Option<&str>,
        change_civil_id_number: Option<&str>,
        change_email: Option<&str>,
        change_date_of_birth: Option<DateTime<Utc>>,
        change_phone_number: Option<&str>,
        change_photo: Option<Option<&str>>,
    ) -> CreateUser {
        let store_photo = match change_photo {
            Some(new_value) => new_value.map(|potential_slice| String::from(potential_slice)),
            None => self.photo.clone(), // original data remains
        };

        // Create a new edit structure
        // `store_change` stores any change a user might wanted to apply
        CreateUser::edit(
            // User::store_change(&self.first_name, &change_first_name),
            change_first_name.map_or_else(
                || self.first_name.clone(),
                |new_first_name| String::from(new_first_name),
            ),
            change_last_name.map_or_else(
                || self.last_name.clone(),
                |new_last_name| String::from(new_last_name),
            ),
            change_civil_id_number.map_or_else(
                || self.civil_id_number.clone(),
                |new_civil_id_number| String::from(new_civil_id_number),
            ),
            change_email.map_or_else(|| self.email.clone(), |new_email| String::from(new_email)),
            change_date_of_birth.map_or_else(
                || self.date_of_birth.clone(),
                |new_date_of_birth| new_date_of_birth.to_string(),
            ),
            change_phone_number.map_or_else(
                || self.phone_number.clone(),
                |new_phone_number| String::from(new_phone_number),
            ),
            self.created_at.clone(),
            store_photo,
        )
    }
}

impl CreateUser {
    /// Creating the write structure for user record
    ///
    /// Params
    /// ---
    /// - first_name: first name of the user
    /// - last_name: last name of the user
    /// - civil_id_number: user's civil id number
    /// - email: user's email
    /// - date_of_birth: user's birth date
    /// - phone_number: user's phone number
    /// - photo: optional - url to the photo
    ///
    /// Returns
    /// ---
    /// - new CreateUser structure (used for database insertion)
    pub fn new(
        first_name: &str,
        last_name: &str,
        civil_id_number: &str,
        email: &str,
        date_of_birth: DateTime<Utc>,
        phone_number: &str,
        photo: Option<&str>,
    ) -> CreateUser {
        CreateUser {
            first_name: String::from(first_name),
            last_name: String::from(last_name),
            civil_id_number: String::from(civil_id_number),
            email: String::from(email),
            date_of_birth: date_of_birth.to_string(),
            phone_number: String::from(phone_number),
            created_at: CurrentTime::store(),
            photo: match photo {
                Some(content) => Some(String::from(content)),
                None => None,
            },
        }
    }

    /// Function used when editing a record -> creating a new edit structure to
    /// edit the database record
    ///
    ///
    /// Params
    /// ---
    /// - first_name: first name of the user
    /// - last_name: last name of the user
    /// - civil_id_number: user's civil id number
    /// - email: user's email
    /// - phone_number: user's phone number
    /// - created_at: allowing the original creation time to be untouched
    /// - photo: optional - url to the photo
    ///
    /// Returns
    /// ---
    /// - new CreateUser structure (used for database UPDATE)
    fn edit(
        first_name: String,
        last_name: String,
        civil_id_number: String,
        email: String,
        date_of_birth: String,
        phone_number: String,
        created_at: String,
        photo: Option<String>,
    ) -> CreateUser {
        CreateUser {
            first_name,
            last_name,
            civil_id_number,
            email,
            date_of_birth,
            phone_number,
            created_at,
            photo,
        }
    }
}
