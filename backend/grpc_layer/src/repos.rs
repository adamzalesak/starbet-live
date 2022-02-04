use std::convert::*;
use std::{collections::HashMap, sync::Arc};

use crate::game_match::{GameEventType, Match};
use crate::team::Team;

use database_layer::{
    connection::PgPool,
    db_access::{
        bet_and_ticket::PgBetAndTicketRepo,
        game::PgGameRepo,
        game_match::{MatchRepo, PgMatchRepo},
        repo::Repo,
        submitted_bet_and_ticket::PgSubmittedBetAndTicketRepo,
        team::{PgTeamRepo, TeamRepo},
        user::PgUserRepo,
    },
    db_models::game_match_event::GameMatchEventType,
};

pub struct Repos {
    pub game: PgGameRepo,
    pub user: PgUserRepo,
    pub game_match: PgMatchRepo,
    pub team: PgTeamRepo,
    pub bet_ticket: PgBetAndTicketRepo,
    pub sub_bet_ticket: PgSubmittedBetAndTicketRepo,
}

impl Repos {
    pub fn new(pool: &Arc<PgPool>) -> Repos {
        Repos {
            game: PgGameRepo::new(pool),
            user: PgUserRepo::new(pool),
            game_match: PgMatchRepo::new(pool),
            team: PgTeamRepo::new(pool),
            bet_ticket: PgBetAndTicketRepo::new(pool),
            sub_bet_ticket: PgSubmittedBetAndTicketRepo::new(pool),
        }
    }

    pub async fn get_filled_match(&self, match_id: i32) -> anyhow::Result<Match> {
        let (game_match, game_event_type) = self.game_match.get_show_info(match_id).await?;

        let mut winner_id = None;
        let grpc_event_type;
        match game_event_type.extract_event()? {
            GameMatchEventType::Upcoming => grpc_event_type = GameEventType::Upcoming,
            GameMatchEventType::Live => grpc_event_type = GameEventType::Live,
            GameMatchEventType::Ended(id) => {
                winner_id = Some(id);
                grpc_event_type = GameEventType::Ended;
            }
            _ => anyhow::bail!("Unsupported event type used"),
        };

        let mut teams = HashMap::new();
        for team_id in vec![game_match.team_one_id, game_match.team_two_id] {
            if !teams.contains_key(&team_id) {
                let team = self.team.get(team_id).await?;
                teams.insert(team_id, team);
            }
        }
        let mut grpc_match = Match::from(&game_match);
        grpc_match.team_one = Some(Team::from(
            teams.get(&game_match.team_one_id).unwrap().clone(),
        ));
        grpc_match.team_two = Some(Team::from(
            teams.get(&game_match.team_two_id).unwrap().clone(),
        ));
        grpc_match.game_event_type = grpc_event_type.into();
        grpc_match.winner_id = winner_id;
        Ok(grpc_match)
    }

    pub async fn change_ratios(&self, match_id: i32, team_id: i32) -> anyhow::Result<()> {
        let (mut ratio1, mut ratio2) = self.game_match.get_ratios(match_id).await?;
        let (game_match, _) = self.game_match.get_show_info(match_id).await?;

        if game_match.team_one_id == team_id {
            ratio1 *= 0.95;
            ratio2 *= 1.1;
        } else {
            ratio1 *= 1.1;
            ratio2 *= 0.95;
        }
        ratio1 = (ratio1 * 100.0).round() / 100.0;
        ratio2 = (ratio2 * 100.0).round() / 100.0;
        self.game_match.set_ratios(match_id, ratio1, ratio2).await?;
        Ok(())
    }
}
