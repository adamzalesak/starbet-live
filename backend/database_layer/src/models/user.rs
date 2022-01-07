use crate::schema::user;

#[derive(Queryable)]
pub struct User {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub civil_id_number: String,
    pub email: String,
    pub phone_number: String,
    pub photo: Option<String>,
}

#[derive(Insertable)]
#[table_name = "user"]
pub struct CreateUser<'a> {
    pub first_name: &'a str,
    pub last_name: &'a str,
    pub civil_id_number: &'a str,
    pub email: &'a str,
    pub phone_number: &'a str,
    pub photo: Option<&'a str>,
}
