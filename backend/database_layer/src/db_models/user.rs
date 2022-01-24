use crate::schema::user;
use crate::type_storing::time_handling::CurrentTime;

/// Read structure, used for data mapping of
/// User record from the database
#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub civil_id_number: String,
    pub email: String,
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
    /// - change_phone_number: ability to change the phone number
    /// - change_photo: ability to change the profile photo
    ///
    pub fn edit_user(
        &self,
        change_first_name: Option<String>,
        change_last_name: Option<String>,
        change_civil_id_number: Option<String>,
        change_email: Option<String>,
        change_phone_number: Option<String>,
        change_photo: Option<Option<String>>,
    ) -> CreateUser {
        let store_photo = match change_photo {
            Some(new_value) => new_value,
            None => None,
        };

        // Create a new edit structure
        // `store_change` stores any change a user might wanted to apply
        CreateUser::edit(
            User::store_change(&self.first_name, &change_first_name),
            User::store_change(&self.last_name, &change_last_name),
            User::store_change(&self.civil_id_number, &change_civil_id_number),
            User::store_change(&self.email, &change_email),
            User::store_change(&self.phone_number, &change_phone_number),
            self.created_at.clone(),
            store_photo,
        )
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
    fn store_change(original_parameter: &str, new_parameter: &Option<String>) -> String {
        match new_parameter {
            Some(parameter) => parameter.clone(),
            None => String::from(original_parameter),
        }
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
        phone_number: &str,
        photo: Option<String>,
    ) -> CreateUser {
        CreateUser {
            first_name: String::from(first_name),
            last_name: String::from(last_name),
            civil_id_number: String::from(civil_id_number),
            email: String::from(email),
            phone_number: String::from(phone_number),
            created_at: CurrentTime::store(),
            photo,
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
        phone_number: String,
        created_at: String,
        photo: Option<String>,
    ) -> CreateUser {
        CreateUser {
            first_name,
            last_name,
            civil_id_number,
            email,
            phone_number,
            created_at,
            photo,
        }
    }
}
