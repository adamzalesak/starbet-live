use crate::schema::UserAddress;

#[derive(Queryable)]
struct GetUserAddress {
    id: i32,
    user_id: i32,
    street_name: String,
    city: String,
    area: String,
    postal_code: String,
    country: String,
    valid_from: String,
}


#[derive(Insertable)]
#[table_name="UserAddress"]
struct NewUserAddress<'a> {
    id: i32,
    user_id: i32,
    street_name: &'a str,
    city: &'a str,
    area: &'a str,
    postal_code: &'a str,
    country: &'a str,
    valid_from: &'a str,
}