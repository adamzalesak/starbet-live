use bytes::{BufMut, BytesMut};
use prost::Message;
use std::sync::Arc;
use tonic::{Code, Request, Response, Status};

use crate::bet::bet_service_server::BetService;
use crate::bet::{
    CreateBetReply, CreateBetRequest, DeleteBetReply, DeleteBetRequest, ListBetsReply,
    ListBetsRequest, ListTicketBetsRequest,
};

use database_layer::{connection::PgPool, db_access::repo::Repo, db_models::game::CreateGame};
use ws_layer::Clients;

pub struct MyBetService {
    //    repo: PgBetRepo,
    ws_clients: Clients,
}

impl MyBetService {
    pub fn new(pool: &Arc<PgPool>, ws_clients: Clients) -> MyBetService {
        MyBetService {
            //           repo: PgBetRepo::new(pool),
            ws_clients: ws_clients,
        }
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

        let mut buf = BytesMut::with_capacity(64);
        /*
        CreateGameReply { id: 1 }.encode(&mut buf);
        for client in self.ws_clients.lock().await.values() {
            if let Some(sender) = &client.sender {
                sender.send(Ok(ws_layer::Msg::binary(buf.clone().freeze().to_vec())));
            }
        }
        */
        let reply = CreateBetReply { id: 0 };
        Ok(Response::new(reply))
    }

    async fn delete_bet(
        &self,
        request: Request<DeleteBetRequest>,
    ) -> Result<Response<DeleteBetReply>, Status> {
        println!("[Server] Request from client: {:?}", &request);

        Ok(Response::new(DeleteBetReply {}))
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
