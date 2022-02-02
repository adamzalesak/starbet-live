use bytes::BytesMut;
use chrono::Utc;
use prost::Message;
use std::convert::*;
use std::{collections::HashMap, sync::Arc};
use tonic::{Code, Request, Response, Status};

use crate::game_match::match_service_server::MatchService;
use crate::game_match::{
    CreateGameEventReply, CreateGameEventRequest, CreateMatchReply, CreateMatchRequest,
    GameEventType, ListMatchesReply, ListMatchesRequest, Match,
};
use crate::team::Team;

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
use ws_layer::Clients;

pub struct MyMatchService {
    repo: PgMatchRepo,
    team_repo: PgTeamRepo,
    ws_clients: Clients,
}

impl MyMatchService {
    pub fn new(pool: &Arc<PgPool>, ws_clients: Clients) -> MyMatchService {
        MyMatchService {
            repo: PgMatchRepo::new(pool),
            team_repo: PgTeamRepo::new(pool),
            ws_clients: ws_clients,
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
            GameEventType::Ended => GameMatchEventFilter::Ended,
        };

        match self
            .repo
            .get_all_show_info(Some(game_match_event_type), None)
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
        let winner_id = request.winner_id;
        let game_match_event_type = match GameEventType::from_i32(request.game_event_type).unwrap()
        {
            GameEventType::Upcoming => GameMatchEventType::Upcoming,
            GameEventType::Live => GameMatchEventType::Live(Utc::now()),
            GameEventType::Ended => GameMatchEventType::Ended(winner_id.unwrap()),
        };
        match self
            .repo
            .create_event(request.match_id, game_match_event_type)
            .await
        {
            Ok(_) => match self.repo.get(request.match_id).await {
                Ok(game_match) => {
                    let mut teams = HashMap::new();
                    for team_id in vec![game_match.team_one_id, game_match.team_two_id] {
                        if !teams.contains_key(&team_id) {
                            let team = match self.team_repo.get(team_id).await {
                                Ok(team) => Ok(Team::from(&team)),
                                Err(err) => Err(Status::new(Code::from_i32(13), err.to_string())),
                            }?;
                            teams.insert(team_id, team);
                        }
                    }

                    let mut grpc_match = Match::from(&game_match);
                    grpc_match.team_one = Some(teams.get(&game_match.team_one_id).unwrap().clone());
                    grpc_match.team_two = Some(teams.get(&game_match.team_two_id).unwrap().clone());
                    grpc_match.game_event_type = request.game_event_type;
                    grpc_match.winner_id = winner_id;

                    let mut buf = BytesMut::with_capacity(64);
                    let _ = grpc_match.encode(&mut buf);
                    for client in self.ws_clients.lock().await.values() {
                        if let Some(sender) = &client.sender {
                            let _ = sender
                                .send(Ok(ws_layer::Msg::binary(buf.clone().freeze().to_vec())));
                        }
                    }
                    Ok(Response::new(CreateGameEventReply {}))
                }
                Err(err) => Err(Status::new(Code::from_i32(13), err.to_string())),
            },
            Err(err) => Err(Status::new(Code::from_i32(13), err.to_string())),
        }
    }
}
