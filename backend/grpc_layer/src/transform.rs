use std::convert::*;

use crate::bet;
use crate::game;
use crate::game_match::{GameEventType, Match};
use crate::team;

use database_layer::db_models::{bet::Bet, game_match::GameMatch, team::Team};
use database_layer::result_types::GameInfo;

impl From<&'_ GameInfo> for game::Game {
    fn from(game: &'_ GameInfo) -> Self {
        game::Game {
            id: game.id,
            name: String::from(&game.name),
            logo_url: String::from(&game.logo_url),
        }
    }
}

impl From<&'_ GameMatch> for Match {
    fn from(game_match: &'_ GameMatch) -> Self {
        Match {
            id: game_match.id,
            game_id: game_match.game_id,
            team_one: None,
            team_two: None,
            team_one_ratio: game_match.team_one_ratio.clone(),
            team_two_ratio: game_match.team_two_ratio.clone(),
            supposed_start_at: game_match.supposed_start_at.clone(),
            state: game_match.state.clone(),
            winner_id: None,
            game_event_type: GameEventType::Upcoming.into(),
        }
    }
}

impl From<&'_ Team> for team::Team {
    fn from(team: &'_ Team) -> Self {
        team::Team {
            id: team.id,
            name: team.name.clone(),
            description: team.description.clone(),
            logo: team.logo.clone(),
        }
    }
}

impl From<&'_ Bet> for bet::Bet {
    fn from(bet: &'_ Bet) -> Self {
        bet::Bet {
            id: bet.id,
            match_id: bet.game_match_id,
            ticket_id: bet.ticket_id,
            team_id: bet.team_id,
        }
    }
}
