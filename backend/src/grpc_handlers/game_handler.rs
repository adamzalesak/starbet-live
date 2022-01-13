use tonic::{Request, Response, Status};

use crate::game::game_service_server::GameService;
use crate::game::{CreateGameReply, CreateGameRequest, DeleteGameRequest, ListGamesReply};

pub struct MyGameService {}

impl MyGameService {
    pub fn new() -> MyGameService {
        MyGameService {}
    }
}

#[tonic::async_trait]
impl GameService for MyGameService {
    async fn list_games(&self, request: Request<()>) -> Result<Response<ListGamesReply>, Status> {
        println!("[Server] Request from client: {:?}", &request);

        let reply = ListGamesReply { games: vec![] };
        Ok(Response::new(reply))
    }

    async fn create_game(
        &self,
        request: Request<CreateGameRequest>,
    ) -> Result<Response<CreateGameReply>, Status> {
        println!("[Server] Request from client: {:?}", &request);

        let reply = CreateGameReply { id: 0 };
        Ok(Response::new(reply))
    }

    async fn delete_game(
        &self,
        request: Request<DeleteGameRequest>,
    ) -> Result<Response<()>, Status> {
        println!("[Server] Request from client: {:?}", &request);

        Ok(Response::new(()))
    }
}
