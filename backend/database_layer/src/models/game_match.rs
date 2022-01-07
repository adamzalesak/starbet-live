use crate::schema::{game_match};
use crate::models::{game::Game};

#[derive(Identifiable, Associations, Queryable, PartialEq)]
#[belongs_to(Team, foreign_key = team_one_id)]
#[belongs_to(Team, foreign_key = team_two_id)]
#[belongs_to(Game)]
#[table_name= "game_match"]
pub struct GameMatch {
    pub id: i32,
    pub game_id: i32,
    pub team_one_id: i32,
    pub team_two_id: i32,
    pub team_one_ratio: String,
    pub team_two_ratio: String,
    pub supposed_start_at: String,
    pub state: String,
}

#[derive(Insertable)]
#[table_name = "game_match"]
pub struct CreateGameMatch<'a> {
    pub game_id: i32,
    pub team_one_id: i32,
    pub team_two_id: i32,
    pub team_one_ratio: &'a str,
    pub team_two_ratio: &'a str,
    pub supposed_start_at: &'a str,
    pub state: &'a str,
}
