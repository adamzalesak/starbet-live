use crate::game_match::match_service_server::MatchService;
use crate::game_match::{
    CreateMatchReply, CreateMatchRequest, DeleteMatchReply, DeleteMatchRequest,
    ListGamesMatchesRequest, ListMatchesReply, ListMatchesRequest, Match,
};

use std::sync::Arc;
use tonic::{Code, Request, Response, Status};

use database_layer::{
    connection::PgPool,
    db_access::{
        game_match::{MatchRepo, PgMatchRepo},
        repo::Repo,
    },
    db_models::game_match::CreateGameMatch,
};

pub struct MyMatchService {
    repo: PgMatchRepo,
}

impl MyMatchService {
    pub fn new(pool: &Arc<PgPool>) -> MyMatchService {
        MyMatchService {
            repo: PgMatchRepo::new(pool),
        }
    }
}

#[tonic::async_trait]
impl MatchService for MyMatchService {
    async fn list_matches(
        &self,
        _: Request<ListMatchesRequest>,
    ) -> Result<Response<ListMatchesReply>, Status> {
        match self.repo.get_all(None, None, None).await {
            Ok(game_matches) => Ok(Response::new(ListMatchesReply {
                game_matches: game_matches
                    .iter()
                    .map(|game_match| Match {
                        // TODO
                        game_id: 1,
                        id: 1,
                        team_one_id: 1,
                        team_two_id: 1,
                        team_one_ratio: 1.0,
                        team_two_ratio: 1.0,
                        start_ts: 1,
                    })
                    .collect(),
            })),
            Err(err) => Err(Status::new(Code::from_i32(13), err.to_string())),
        }
    }

    async fn create_match(
        &self,
        request: Request<CreateMatchRequest>,
    ) -> Result<Response<CreateMatchReply>, Status> {
        let request = request.into_inner();
        let create_match = CreateGameMatch {
            game_id: request.game_id,
            team_one_id: request.team_one_id,
            team_two_id: request.team_two_id,
            team_one_ratio: "", //request.team_one_ratio,
            team_two_ratio: "", //request.team_two_ratio,
            supposed_start_at: "",
            state: "",
        };

        /*
        match self.repo.create(create_match).await {
            Ok(match_id) => Ok(Response::new(CreateMatchReply { id: match_id })),
            Err(err) => Err(Status::new(Code::from_i32(13), err.to_string())),
        }
        */
        Ok(Response::new(CreateMatchReply { id: 1 }))
    }

    async fn delete_match(
        &self,
        request: Request<DeleteMatchRequest>,
    ) -> Result<Response<DeleteMatchReply>, Status> {
        println!("[Server] Request from client: {:?}", &request);

        Ok(Response::new(DeleteMatchReply {}))
    }

    async fn list_games_matches(
        &self,
        request: Request<ListGamesMatchesRequest>,
    ) -> Result<Response<ListMatchesReply>, Status> {
        // TODO: get_all by game_id
        match self.repo.get_all(None, None, None).await {
            Ok(game_matches) => Ok(Response::new(ListMatchesReply {
                game_matches: game_matches
                    .iter()
                    .map(|game_match| Match {
                        game_id: 1,
                        id: 1,
                        team_one_id: 1,
                        team_two_id: 1,
                        team_one_ratio: 1.0,
                        team_two_ratio: 1.0,
                        start_ts: 1,
                    })
                    .collect(),
            })),
            Err(err) => Err(Status::new(Code::from_i32(13), err.to_string())),
        }
    }
}
