use crate::game_match::match_service_server::MatchService;
use crate::game_match::{
    CreateGameEventReply, CreateGameEventRequest, CreateMatchReply, CreateMatchRequest,
    GameEventType, ListMatchesReply, ListMatchesRequest, Match,
};
use crate::team::Team;
use std::convert::*;

use chrono::Utc;
use std::{collections::HashMap, sync::Arc};
use tonic::{Code, Request, Response, Status};

use database_layer::{
    connection::PgPool,
    db_access::{
        game_match::{MatchRepo, PgMatchRepo},
        repo::Repo,
        team::{PgTeamRepo, TeamRepo},
    },
    db_models::{
        game_match::CreateGameMatch,
        game_match_event::{GameMatchEventFilter, GameMatchEventType},
    },
};

pub struct MyMatchService {
    repo: PgMatchRepo,
    team_repo: PgTeamRepo,
}

impl MyMatchService {
    pub fn new(pool: &Arc<PgPool>) -> MyMatchService {
        MyMatchService {
            repo: PgMatchRepo::new(pool),
            team_repo: PgTeamRepo::new(pool),
        }
    }
}

#[tonic::async_trait]
impl MatchService for MyMatchService {
    async fn list_matches(
        &self,
        request: Request<ListMatchesRequest>,
    ) -> Result<Response<ListMatchesReply>, Status> {
        let request = request.into_inner();
        let game_match_event_type = match GameEventType::from_i32(request.game_event_type).unwrap()
        {
            GameEventType::Upcoming => GameMatchEventFilter::Upcoming,
            GameEventType::Live => GameMatchEventFilter::Live,
            GameEventType::Ended => GameMatchEventFilter::Ended, // TODO
        };

        match self
            .repo
            .get_all_show_info(Some(game_match_event_type), Some(request.game_id))
            .await
        {
            Ok(game_matches) => {
                let mut teams = HashMap::new();
                for (game_match, _) in &game_matches {
                    for team_id in vec![game_match.team_one_id, game_match.team_two_id] {
                        if !teams.contains_key(&team_id) {
                            let team = match self.team_repo.get(team_id).await {
                                Ok(team) => Ok(Team::from(&team)),
                                Err(err) => Err(Status::new(Code::from_i32(13), err.to_string())),
                            }?;
                            teams.insert(team_id, team);
                        }
                    }
                }

                Ok(Response::new(ListMatchesReply {
                    game_matches: game_matches
                        .iter()
                        .map(|(game_match, _)| {
                            let mut grpc_match = Match::from(game_match);
                            grpc_match.team_one =
                                Some(teams.get(&game_match.team_one_id).unwrap().clone());
                            grpc_match.team_two =
                                Some(teams.get(&game_match.team_two_id).unwrap().clone());
                            grpc_match
                        })
                        .collect(),
                }))
            }
            Err(err) => Err(Status::new(Code::from_i32(13), err.to_string())),
        }
    }

    async fn create_match(
        &self,
        request: Request<CreateMatchRequest>,
    ) -> Result<Response<CreateMatchReply>, Status> {
        let request = request.into_inner();
        let create_match = CreateGameMatch::new(
            request.game_id,
            request.team_one_id,
            request.team_two_id,
            &*request.team_one_ratio,
            &*request.team_two_ratio,
            Utc::now(),
            &*request.state,
        );

        match self.repo.create(create_match).await {
            Ok(match_id) => Ok(Response::new(CreateMatchReply { id: match_id })),
            Err(err) => Err(Status::new(Code::from_i32(13), err.to_string())),
        }
    }

    async fn create_game_event(
        &self,
        request: Request<CreateGameEventRequest>,
    ) -> Result<Response<CreateGameEventReply>, Status> {
        let request = request.into_inner();
        let game_match_event_type = match GameEventType::from_i32(request.game_event_type).unwrap()
        {
            GameEventType::Upcoming => GameMatchEventType::Upcoming,
            GameEventType::Live => GameMatchEventType::Live(Utc::now()),
            GameEventType::Ended => GameMatchEventType::Ended(0), // TODO
        };
        match self
            .repo
            .create_event(request.match_id, game_match_event_type)
            .await
        {
            Ok(_) => Ok(Response::new(CreateGameEventReply {})),
            Err(err) => Err(Status::new(Code::from_i32(13), err.to_string())),
        }
    }
}
