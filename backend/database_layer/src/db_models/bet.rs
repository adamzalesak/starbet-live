use crate::db_models::{game_match::GameMatch, team::Team, ticket::Ticket};
use crate::schema::bet;

#[derive(Identifiable, Queryable, Associations, PartialEq)]
#[belongs_to(Ticket)]
#[belongs_to(Team)]
#[belongs_to(GameMatch)]
#[table_name = "bet"]
pub struct Bet {
    pub id: i32,
    pub game_match_id: i32,
    pub ticket_id: i32,
    pub team_id: i32,
    pub bet_ratio: String,
    pub bet_price: String,
    pub created_at: String,
}

#[derive(Insertable)]
#[table_name = "bet"]
pub struct CreateBet<'a> {
    pub game_match_id: i32,
    pub ticket_id: i32,
    pub team_id: i32,
    pub bet_ratio: &'a str,
    pub bet_price: &'a str,
    pub created_at: &'a str,
}
