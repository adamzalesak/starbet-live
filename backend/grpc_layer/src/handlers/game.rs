use std::convert::*;
use std::sync::Arc;
use tonic::{Code, Request, Response, Status};

use crate::game::game_service_server::GameService;
use crate::game::{CreateGameReply, CreateGameRequest, Game, ListGamesReply, ListGamesRequest};

use database_layer::{
    connection::PgPool,
    db_access::{
        game::{GameRepo, PgGameRepo},
        repo::Repo,
    },
    db_models::game::CreateGame,
};

pub struct MyGameService {
    repo: PgGameRepo,
}

impl MyGameService {
    pub fn new(pool: &Arc<PgPool>) -> MyGameService {
        MyGameService {
            repo: PgGameRepo::new(pool),
        }
    }
}

#[tonic::async_trait]
impl GameService for MyGameService {
    async fn list_games(
        &self,
        _: Request<ListGamesRequest>,
    ) -> Result<Response<ListGamesReply>, Status> {
        match self.repo.get_all().await {
            Ok(game_infos) => Ok(Response::new(ListGamesReply {
                games: game_infos.iter().map(|game| Game::from(game)).collect(),
            })),
            Err(err) => Err(Status::new(Code::from_i32(13), err.to_string())),
        }
    }

    async fn create_game(
        &self,
        request: Request<CreateGameRequest>,
    ) -> Result<Response<CreateGameReply>, Status> {
        let request = request.into_inner();
        let create_game = CreateGame::new(&*request.name, "", &*request.logo_url); // TODO

        match self.repo.create(create_game).await {
            Ok(game_id) => Ok(Response::new(CreateGameReply { id: game_id })),
            Err(err) => Err(Status::new(Code::from_i32(13), err.to_string())),
        }
    }
}
