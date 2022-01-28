use crate::schema::game;

/// Read structure, used for data mapping of
/// `game` record from the database
#[derive(Queryable)]
pub struct Game {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub logo_url: String,
}

/// Write structure, used for inserting
/// `game` records into the database
#[derive(Insertable, AsChangeset)]
#[table_name = "game"]
pub struct CreateGame {
    pub name: String,
    pub description: String,
    pub logo: String,
}

impl CreateGame {
    /// Create a new `game`` insert structure
    ///
    /// Params
    /// ---
    /// - name: name of the game
    /// - description: description of the game
    /// - logo: path to the logo of the game
    ///
    /// Returns
    /// ---
    /// - new `game` insert structure
    pub fn new(name: &str, description: &str, logo: &str) -> CreateGame {
        CreateGame {
            name: String::from(name),
            description: String::from(description),
            logo: String::from(logo),
        }
    }
}
