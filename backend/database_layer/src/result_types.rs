use diesel::sql_types::{Integer, Text};

/// Structure used for getting some fields of Game records from the database
/// This is to limit the amout of traffic between the db and the backend
pub struct GameInfo {
    pub id: i32,
    pub name: String,
    pub logo_url: String,
}

impl GameInfo {
    /// Create a new game info structure
    ///
    /// Params
    /// ---
    /// - id: game's ID
    /// - name: game's name
    /// - logo_url: game's logo
    ///
    /// Returns
    /// ---
    /// - new game info structure
    pub fn new(id: i32, name: String, logo_url: String) -> Self {
        Self { id, name, logo_url }
    }

    /// Create a vector of GameInfo from GameInfoRetrieve
    ///
    /// Params
    /// ---
    /// - teams: slice of GameInfoRetrieve
    ///
    /// Returns
    /// ---
    /// - vector of GameInfo
    pub fn from_vector(teams: &[GameInfoRetrieve]) -> Vec<GameInfo> {
        teams
            .iter()
            .map(|(id, name, logo_url)| Self::new(*id, name.clone(), logo_url.clone()))
            .collect()
    }
}

/// type alias for fields, that are retrieved from the db
pub type GameInfoRetrieve = (i32, String, String);

/// Structure used for getting some fields of Team records from the database
/// This is to limit the amout of traffic between the db and the backend
pub struct TeamInfo {
    pub id: i32,
    pub name: String,
    pub logo_url: String,
}

impl TeamInfo {
    /// Create a new team info structure
    ///
    /// Params
    /// ---
    /// - id: team's ID
    /// - name: team's name
    /// - logo_url: team's logo
    ///
    /// Returns
    /// ---
    /// - new team info structure
    pub fn new(id: i32, name: String, logo_url: String) -> Self {
        Self { id, name, logo_url }
    }

    /// Create a vector of TeamInfo from TeamInfoRetrieve
    ///
    /// Params
    /// ---
    /// - teams: slice of TeamInfoRetrieve
    ///
    /// Returns
    /// ---
    /// - vector of TeamInfo
    pub fn from_vector(teams: &[TeamInfoRetrieve]) -> Vec<TeamInfo> {
        teams
            .iter()
            .map(|(id, name, logo_url)| Self::new(*id, name.clone(), logo_url.clone()))
            .collect()
    }
}

/// type alias for fields, that are retrieved from the db
pub type TeamInfoRetrieve = (i32, String, String);

#[derive(QueryableByName, Debug)]
pub struct GameMatchShow {
    #[sql_type = "Integer"]
    pub id: i32,
    #[sql_type = "Integer"]
    pub game_id: i32,
    #[sql_type = "Integer"]
    pub team_one_id: i32,
    #[sql_type = "Integer"]
    pub team_two_id: i32,
    #[sql_type = "Text"]
    pub team_one_ratio: String,
    #[sql_type = "Text"]
    pub team_two_ratio: String,
    #[sql_type = "Text"]
    pub supposed_start_at: String,
    #[sql_type = "Text"]
    pub state: String,
    #[sql_type = "Text"]
    pub event_type: String,
    #[sql_type = "Text"]
    pub team_one_name: String,
    #[sql_type = "Text"]
    pub team_two_name: String,
    #[sql_type = "Text"]
    pub games_name: String,
}
