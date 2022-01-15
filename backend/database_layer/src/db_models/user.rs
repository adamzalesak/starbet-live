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
#[derive(Insertable)]
#[table_name = "user"]
pub struct CreateUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub civil_id_number: &'a str,
    pub email: &'a str,
    pub phone_number: &'a str,
    pub created_at: String,
    pub photo: Option<&'a str>,
}

impl<'a> CreateUser<'a> {
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
        first_name: &'a str,
        last_name: &'a str,
        civil_id_number: &'a str,
        email: &'a str,
        phone_number: &'a str,
        photo: Option<&'a str>,
    ) -> CreateUser<'a> {
        CreateUser {
            first_name,
            last_name,
            civil_id_number,
            email,
            phone_number,
            created_at: CurrentTime::store(),
            photo,
        }
    }
}
