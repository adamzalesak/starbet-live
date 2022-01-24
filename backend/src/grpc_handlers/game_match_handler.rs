use tonic::{Request, Response, Status};

use crate::game_match::game_match_service_server::GameMatchService;
use crate::game_match::{
    CreateGameMatchReply, CreateGameMatchRequest, DeleteGameMatchRequest, ListGameMatchesReply,
    ListGameMatchesRequest,
};

pub struct MyGameMatchService {}

impl MyGameMatchService {
    pub fn new() -> MyGameMatchService {
        MyGameMatchService {}
    }
}

#[tonic::async_trait]
impl GameMatchService for MyGameMatchService {
    async fn list_game_matches(
        &self,
        request: Request<()>,
    ) -> Result<Response<ListGameMatchesReply>, Status> {
        println!("[Server] Request from client: {:?}", &request);

        let reply = ListGameMatchesReply {
            game_matches: vec![],
        };
        Ok(Response::new(reply))
    }

    async fn create_game_match(
        &self,
        request: Request<CreateGameMatchRequest>,
    ) -> Result<Response<CreateGameMatchReply>, Status> {
        println!("[Server] Request from client: {:?}", &request);

        let reply = CreateGameMatchReply { id: 0 };
        Ok(Response::new(reply))
    }

    async fn delete_game_match(
        &self,
        request: Request<DeleteGameMatchRequest>,
    ) -> Result<Response<()>, Status> {
        println!("[Server] Request from client: {:?}", &request);

        Ok(Response::new(()))
    }

    async fn list_game_game_matches(
        &self,
        request: Request<ListGameMatchesRequest>,
    ) -> Result<Response<ListGameMatchesReply>, Status> {
        println!("[Server] Request from client: {:?}", &request);

        let reply = ListGameMatchesReply {
            game_matches: vec![],
        };
        Ok(Response::new(reply))
    }
}
