use crate::schema::team;

#[derive(Queryable)]
pub struct Team {
    id: i32,
    name: String,
    description: String,
    logo: String,
}

#[derive(Insertable)]
#[table_name = "team"]
pub struct CreateTeam<'a> {
    name: &'a str,
    description: &'a str,
    logo: &'a str,
}
