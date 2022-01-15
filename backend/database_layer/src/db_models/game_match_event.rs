use crate::db_models::game_match::GameMatch;
use crate::schema::game_match_event;

#[derive(Identifiable, Associations, Queryable, PartialEq)]
#[belongs_to(GameMatch)]
#[table_name = "game_match_event"]
pub struct GameMatchEvent {
    pub id: i32,
    pub game_match_id: i32,
    pub event_type: String,
    pub created_at: String,
    pub overtime_until: Option<String>,
}

#[derive(Insertable)]
#[table_name = "game_match_event"]
pub struct CreateGameMatchEvent<'a> {
    pub game_match_id: i32,
    pub event_type: &'a str,
    pub created_at: &'a str,
}
