use tonic::{Request, Response, Status};

use crate::bet::bet_service_server::BetService;
use crate::bet::{
    CreateBetReply, CreateBetRequest, DeleteBetRequest, ListBetsReply, ListBetsRequest,
    ListTicketBetsRequest,
};

pub struct MyBetService {}

impl MyBetService {
    pub fn new() -> MyBetService {
        MyBetService {}
    }
}

#[tonic::async_trait]
impl BetService for MyBetService {
    async fn list_bets(
        &self,
        request: Request<ListBetsRequest>,
    ) -> Result<Response<ListBetsReply>, Status> {
        println!("[Server] Request from client: {:?}", &request);

        let reply = ListBetsReply { bets: vec![] };
        Ok(Response::new(reply))
    }

    async fn create_bet(
        &self,
        request: Request<CreateBetRequest>,
    ) -> Result<Response<CreateBetReply>, Status> {
        println!("[Server] Request from client: {:?}", &request);

        let reply = CreateBetReply { id: 0 };
        Ok(Response::new(reply))
    }

    async fn delete_bet(&self, request: Request<DeleteBetRequest>) -> Result<Response<()>, Status> {
        println!("[Server] Request from client: {:?}", &request);

        Ok(Response::new(()))
    }

    async fn list_ticket_bets(
        &self,
        request: Request<ListTicketBetsRequest>,
    ) -> Result<Response<ListBetsReply>, Status> {
        println!("[Server] Request from client: {:?}", &request);

        let reply = ListBetsReply { bets: vec![] };
        Ok(Response::new(reply))
    }
}
