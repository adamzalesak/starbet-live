use crate::schema::game;

#[derive(Queryable)]
pub struct Game {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub logo: String,
}

#[derive(Insertable)]
#[table_name = "game"]
pub struct CreateGame<'a> {
    pub name: &'a str,
    pub description: &'a str,
    pub logo: &'a str,
}
