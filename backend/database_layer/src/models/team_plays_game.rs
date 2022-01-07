use crate::models::{game::Game, team::Team};
use crate::schema::team_plays_game;

#[derive(Identifiable, Associations, Queryable, PartialEq)]
#[belongs_to(Team)]
#[belongs_to(Game)]
#[table_name = "team_plays_game"]
pub struct TeamPlaysGame {
    pub id: i32,
    pub team_id: i32,
    pub game_id: i32,
}

#[derive(Insertable)]
#[table_name = "team_plays_game"]
pub struct CreateTeamPlaysGame {
    pub team_id: i32,
    pub game_id: i32,
}
