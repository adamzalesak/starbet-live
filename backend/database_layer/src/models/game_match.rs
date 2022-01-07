use crate::schema::{game_match};
use crate::models::{game::Game};

#[derive(Identifiable, Associations, Queryable, PartialEq)]
#[belongs_to(Team, foreign_key = team_one_id)]
#[belongs_to(Team, foreign_key = team_two_id)]
#[belongs_to(Game)]
#[table_name= "game_match"]
pub struct GameMatch {
    id: i32,
    game_id: i32,
    team_one_id: i32,
    team_two_id: i32,
    team_one_ratio: String,
    team_two_ratio: String,
    supposed_start_at: String,
    state: String,
}

#[derive(Insertable)]
#[table_name = "game_match"]
pub struct CreateGameMatch<'a> {
    game_id: i32,
    team_one_id: i32,
    team_two_id: i32,
    team_one_ratio: &'a str,
    team_two_ratio: &'a str,
    supposed_start_at: &'a str,
    state: &'a str,
}
