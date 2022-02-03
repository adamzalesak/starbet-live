use crate::schema::user;
use crate::type_storing::time_handling::TimeHandling;

/// Read structure, used for data mapping of
/// `user` record from the database
#[derive(Queryable, Clone)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub user_password: String,
    pub civil_id_number: String,
    pub date_of_birth: String,
    pub email: String,
    pub phone_number: String,
    pub created_at: String,
    pub balance: String,
    pub photo: Option<String>,
}

/// Write structure, used for inserting
/// `user` records into the database
#[derive(Insertable, AsChangeset)]
#[table_name = "user"]
pub struct CreateUser {
    pub first_name: String,
    pub last_name: String,
    pub user_password: String,
    pub civil_id_number: String,
    pub date_of_birth: String,
    pub email: String,
    pub phone_number: String,
    pub created_at: String,
    pub balance: String,
    pub photo: Option<String>,
}

impl User {
    /// Create a new `user` update record where each parameter of the user structure apart from the `id`
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
    /// - change_date_of_birth: ability to change the date of birth
    /// - change_email: ability to change the email address
    /// - change_phone_number: ability to change the phone number
    /// - change_photo: ability to change the profile photo
    ///
    /// Returns
    /// ---
    /// - new `user` update structure
    #[allow(clippy::too_many_arguments)]
    pub fn edit_user(
        &self,
        change_first_name: Option<&str>,
        change_last_name: Option<&str>,
        change_password: Option<&str>,
        change_civil_id_number: Option<&str>,
        change_date_of_birth: Option<&str>,
        change_email: Option<&str>,
        change_phone_number: Option<&str>,
        change_photo: Option<Option<&str>>,
    ) -> CreateUser {
        // Create a new update structure
        CreateUser {
            first_name: change_first_name.map_or_else(|| self.first_name.clone(), String::from),
            last_name: change_last_name.map_or_else(|| self.last_name.clone(), String::from),
            user_password: change_password.map_or_else(|| self.user_password.clone(), String::from),
            civil_id_number: change_civil_id_number
                .map_or_else(|| self.civil_id_number.clone(), String::from),
            date_of_birth: change_date_of_birth.map_or_else(
                || self.date_of_birth.clone(),
                |new_date_of_birth| new_date_of_birth.to_string(),
            ),
            email: change_email.map_or_else(|| self.email.clone(), String::from),
            phone_number: change_phone_number
                .map_or_else(|| self.phone_number.clone(), String::from),
            created_at: self.created_at.clone(),
            balance: self.balance.clone(),
            photo: match change_photo {
                Some(new_value) => new_value.map(String::from),
                None => self.photo.clone(), // original data remains
            },
        }
    }
}

impl CreateUser {
    /// Create a new `user` insert structure
    ///
    /// Params
    /// ---
    /// - `first_name`: first name of the user
    /// - `last_name`: last name of the user
    /// - `civil_id_number`: user's civil id number
    /// - `date_of_birth`: user's birth date
    /// - `email`: user's email
    /// - `phone_number`: user's phone number
    /// - `photo`: optional - url to the photo
    ///
    /// Returns
    /// ---
    /// - new `user` insert structure
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        first_name: &str,
        last_name: &str,
        user_password: &str,
        civil_id_number: &str,
        date_of_birth: &str,
        email: &str,
        phone_number: &str,
        photo: Option<&str>,
    ) -> CreateUser {
        CreateUser {
            first_name: String::from(first_name),
            last_name: String::from(last_name),
            user_password: String::from(user_password),
            civil_id_number: String::from(civil_id_number),
            date_of_birth: String::from(date_of_birth),
            email: String::from(email),
            phone_number: String::from(phone_number),
            created_at: TimeHandling::store(),
            balance: 100.0.to_string(),
            photo: photo.map(String::from),
        }
    }
}
