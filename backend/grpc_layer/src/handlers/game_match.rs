use bytes::BytesMut;
use chrono::{DateTime, Utc};
use prost::Message;
use std::convert::*;
use std::{collections::HashMap, sync::Arc};
use tonic::{Code, Request, Response, Status};

use crate::game_match::match_service_server::MatchService;
use crate::game_match::{
    ChangeStateReply, ChangeStateRequest, CreateGameEventReply, CreateGameEventRequest,
    CreateMatchReply, CreateMatchRequest, GameEventType, ListMatchesReply, ListMatchesRequest,
    Match,
};
use crate::repos::Repos;
use crate::team::Team;
use std::convert::*;

use database_layer::{
    connection::PgPool,
    db_access::{game_match::MatchRepo, team::TeamRepo},
    db_models::{
        game_match::CreateGameMatch,
        game_match_event::{GameMatchEventFilter, GameMatchEventType},
    },
};
use ws_layer::Clients;

pub struct MyMatchService {
    repos: Repos,
    ws_clients: Clients,
}

impl MyMatchService {
    pub fn new(pool: &Arc<PgPool>, ws_clients: Clients) -> MyMatchService {
        MyMatchService {
            repos: Repos::new(pool),
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
            .repos
            .game_match
            .get_all_show_info(Some(game_match_event_type), None)
            .await
        {
            Ok(game_matches) => {
                let mut teams = HashMap::new();
                for (game_match, _) in &game_matches {
                    for team_id in vec![game_match.team_one_id, game_match.team_two_id] {
                        if !teams.contains_key(&team_id) {
                            let team = match self.repos.team.get(team_id).await {
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
                        .map(|(game_match, game_event_type)| {
                            let mut winner_id = None;
                            let grpc_event_type = match game_event_type.extract_event().unwrap() {
                                GameMatchEventType::Upcoming => GameEventType::Upcoming,
                                GameMatchEventType::Live => GameEventType::Live,
                                GameMatchEventType::Ended(id) => {
                                    winner_id = Some(id);
                                    GameEventType::Ended
                                }
                                _ => GameEventType::Upcoming,
                            };

                            let mut grpc_match = Match::from(game_match);
                            grpc_match.team_one =
                                Some(teams.get(&game_match.team_one_id).unwrap().clone());
                            grpc_match.team_two =
                                Some(teams.get(&game_match.team_two_id).unwrap().clone());
                            grpc_match.game_event_type = grpc_event_type.into();
                            grpc_match.winner_id = winner_id;
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
            request.supposed_start_at.parse::<DateTime<Utc>>().unwrap(),
            &*request.state,
        );

        match self.repos.game_match.create(create_match).await {
            Ok(match_id) => {
                match self.repos.get_filled_match(match_id).await {
                    Ok(game_match) => {
                        let mut buf = BytesMut::with_capacity(64);
                        let _ = game_match.encode(&mut buf);
                        for client in self.ws_clients.lock().await.values() {
                            if let Some(sender) = &client.sender {
                                let _ = sender
                                    .send(Ok(ws_layer::Msg::binary(buf.clone().freeze().to_vec())));
                            }
                        }
                    }
                    Err(err) => return Err(Status::new(Code::from_i32(13), err.to_string())),
                }
                Ok(Response::new(CreateMatchReply { id: match_id }))
            }
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
            GameEventType::Live => GameMatchEventType::Live,
            GameEventType::Ended => GameMatchEventType::Ended(winner_id.unwrap()),
        };
        match self
            .repos
            .game_match
            .create_event(request.match_id, game_match_event_type)
            .await
        {
            Ok(_) => {
                match self.repos.get_filled_match(request.match_id).await {
                    Ok(game_match) => {
                        let mut buf = BytesMut::with_capacity(64);
                        let _ = game_match.encode(&mut buf);
                        for client in self.ws_clients.lock().await.values() {
                            if let Some(sender) = &client.sender {
                                let _ = sender
                                    .send(Ok(ws_layer::Msg::binary(buf.clone().freeze().to_vec())));
                            }
                        }
                    }
                    Err(err) => return Err(Status::new(Code::from_i32(13), err.to_string())),
                }
                Ok(Response::new(CreateGameEventReply {}))
            }
            Err(err) => Err(Status::new(Code::from_i32(13), err.to_string())),
        }
    }

    async fn change_state(
        &self,
        request: Request<ChangeStateRequest>,
    ) -> Result<Response<ChangeStateReply>, Status> {
        let request = request.into_inner();
        match self
            .repos
            .game_match
            .update_status(request.match_id, &request.state)
            .await
        {
            Ok(()) => {
                match self.repos.get_filled_match(request.match_id).await {
                    Ok(game_match) => {
                        let mut buf = BytesMut::with_capacity(64);
                        let _ = game_match.encode(&mut buf);
                        for client in self.ws_clients.lock().await.values() {
                            if let Some(sender) = &client.sender {
                                let _ = sender
                                    .send(Ok(ws_layer::Msg::binary(buf.clone().freeze().to_vec())));
                            }
                        }
                    }
                    Err(err) => return Err(Status::new(Code::from_i32(13), err.to_string())),
                }
                Ok(Response::new(ChangeStateReply {}))
            }
            Err(err) => Err(Status::new(Code::from_i32(13), err.to_string())),
        }
    }
}
