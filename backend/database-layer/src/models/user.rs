use crate::schema::user;

#[derive(Queryable)]
pub struct User {
    id: i32,
    first_name: String,
    last_name: String,
    civil_id_number: String,
    email: String,
    phone_number: String,
    photo: Option<String>,
}

#[derive(Insertable)]
#[table_name = "user"]
pub struct CreateUser<'a> {
    first_name: &'a str,
    last_name: &'a str,
    civil_id_number: &'a str,
    email: &'a str,
    phone_number: &'a str,
    photo: Option<&'a str>,
}
