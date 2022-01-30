use diesel::sql_types::{Integer, Text};

/// Structure used for getting some fields of Game records from the database
/// This is to limit the amout of traffic between the db and the backend
#[derive(QueryableByName, Queryable)]
pub struct GameInfo {
    #[sql_type = "Integer"]
    pub id: i32,
    #[sql_type = "Text"]
    pub name: String,
    #[sql_type = "Text"]
    pub logo_url: String,
}

/// Structure used for getting some fields of Team records from the database
/// This is to limit the amout of traffic between the db and the backend
#[derive(QueryableByName, Queryable)]
pub struct TeamInfo {
    #[sql_type = "Integer"]
    pub id: i32,
    #[sql_type = "Text"]
    pub name: String,
    #[sql_type = "Text"]
    pub logo_url: String,
}

/// Structure fo retrieving display information for `game_match` records.
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

/// Structure for retrieving display information for `bet` records
#[derive(QueryableByName)]
pub struct BetShowInfo {
    #[sql_type = "Integer"]
    pub id: i32,
    #[sql_type = "Integer"]
    pub game_match_id: i32,
    #[sql_type = "Integer"]
    pub ticket_id: i32,
    #[sql_type = "Integer"]
    pub team_id: i32,
    #[sql_type = "Text"]
    pub bet_ratio: String,
    #[sql_type = "Text"]
    pub bet_price: String,
    #[sql_type = "Text"]
    pub created_at: String,
    #[sql_type = "Text"]
    pub game_name: String,
    #[sql_type = "Text"]
    pub team_name: String,
}

pub struct TicketShowInfo {
    pub id: i32,
    pub user_id: i32,
    pub user_address_id: i32,
    pub created_at: String,
    pub valid_until: String,
    pub price: String,
    pub paid_at: Option<String>,
}
