use crate::schema::bet;
use crate::models::{ticket::Ticket, game_match::GameMatch, team::Team};

#[derive(Identifiable, Queryable, Associations, PartialEq)]
#[belongs_to(Ticket)]
#[belongs_to(Team)]
#[belongs_to(GameMatch)]
#[table_name = "bet"]
pub struct Bet {
    id: i32,
    game_match_id: i32,
    ticket_id: i32,
    team_id: i32,
    bet_ratio: String,
    bet_price: String,
    created_at: String,
}

#[derive(Insertable)]
#[table_name = "bet"]
pub struct CreateBet<'a> {
    game_match_id: i32,
    ticket_id: i32,
    team_id: i32,
    bet_ratio: &'a str,
    bet_price: &'a str,
    created_at: &'a str,
}
