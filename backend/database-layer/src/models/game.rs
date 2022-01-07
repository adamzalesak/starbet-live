use crate::schema::game;

#[derive(Queryable)]
pub struct Game {
    id: i32,
    name: String,
    description: String,
    logo: String,
}

#[derive(Insertable)]
#[table_name = "game"]
pub struct CreateGame<'a> {
    name: &'a str,
    description: &'a str,
    logo: &'a str,
}
