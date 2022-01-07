use crate::schema::team;

#[derive(Queryable)]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub logo: String,
}

#[derive(Insertable)]
#[table_name = "team"]
pub struct CreateTeam<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub logo: &'a str,
}
