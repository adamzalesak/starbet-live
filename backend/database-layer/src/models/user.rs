use crate::schema::User;

#[derive(Queryable)]
struct GetUser {
    id: i32,
    first_name: String,
    last_name: String,
    civil_id_number: String,
    email: String,
    phone_number: String,
    photo: Option<String>,
}


#[derive(Insertable)]
#[table_name="User"]
struct NewUser<'a> {
    id: i32,
    first_name: &'a str,
    last_name: &'a str,
    civil_id_number: &'a str,
    email: &'a str,
    phone_number: &'a str,
    photo: Option<&'a str>,
}