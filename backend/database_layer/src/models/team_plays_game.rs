use crate::schema::team_plays_game;
use crate::models::{team::Team, game::Game};

#[derive(Identifiable, Associations, Queryable, PartialEq)]
#[belongs_to(Team)]
#[belongs_to(Game)]
#[table_name = "team_plays_game"]
pub struct TeamPlaysGame {
    id: i32,
    team_id: i32,
    game_id: i32,
}

#[derive(Insertable)]
#[table_name = "team_plays_game"]
pub struct CreateTeamPlaysGame {
    team_id: i32,
    game_id: i32,
}
