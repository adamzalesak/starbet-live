use crate::schema::game;

#[derive(Queryable)]
pub struct Game {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub logo_url: String,
}

#[derive(Insertable, AsChangeset)]
#[table_name = "game"]
pub struct CreateGame<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub logo: &'a str,
}

impl<'a> CreateGame<'a> {
    pub fn new(name: &'a str, description: &'a str, logo: &'a str) -> CreateGame<'a> {
        CreateGame {
            name,
            description,
            logo,
        }
    }
}
