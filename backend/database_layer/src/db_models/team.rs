use crate::schema::team;

#[derive(Queryable)]
pub struct Team {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub logo: String,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "team"]
pub struct CreateTeam<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub logo: &'a str,
}

impl<'a> CreateTeam<'a> {
    pub fn new(name: &'a str, description: &'a str, logo: &'a str) -> Self {
        Self {
            name,
            description,
            logo,
        }
    }
}
