use crate::schema::game_match_event;
use crate::models::game_match::GameMatch;

#[derive(Identifiable, Associations, Queryable, PartialEq)]
#[belongs_to(GameMatch)]
#[table_name = "game_match_event"]
pub struct GameMatchEvent {
    id: i32,
    game_match_id: i32,
    event_type: String,
    created_at: String,
    overtime_until: Option<String>,
}

#[derive(Insertable)]
#[table_name = "game_match_event"]
pub struct CreateGameMatchEvent<'a> {
    game_match_id: i32,
    event_type: &'a str,
    created_at: &'a str,
}
