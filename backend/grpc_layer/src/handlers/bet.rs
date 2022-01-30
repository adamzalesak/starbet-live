use bytes::{BufMut, BytesMut};
use prost::Message;
use std::sync::Arc;
use tonic::{Code, Request, Response, Status};

use crate::bet::bet_service_server::BetService;
use crate::bet::{
    CreateBetReply, CreateBetRequest, DeleteBetReply, DeleteBetRequest, ListBetsReply,
    ListBetsRequest,
};

use database_layer::{
    connection::PgPool,
    db_access::{
        bet::{BetRepo, PgBetRepo},
        repo::Repo,
    },
    db_models::bet::CreateBet,
};
use ws_layer::Clients;

pub struct MyBetService {
    repo: PgBetRepo,
    ws_clients: Clients,
}

impl MyBetService {
    pub fn new(pool: &Arc<PgPool>, ws_clients: Clients) -> MyBetService {
        MyBetService {
            repo: PgBetRepo::new(pool),
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
        /*
        let request = request.into_inner();
        let create_bet = CreateBet::new(
            request.match_id,
            request.ticket_id,
            request.team_id,
            &request.ratio,
            "Placed",
        );

        match self.repo.place(create_bet).await {
            Ok(bet_id) => {
                let bet = Bet {
                    id: bet_id,
                    game_match_id: create_bet.game_match_id,
                    ticket_id: create_bet.ticket_id,
                    team_id: create_bet.team_id,
                    bet_ratio: create_bet.bet_ratio,
                    bet_state: create_bet.bet_state,
                    created_at: create_bet.created_at,
                };

                let mut buf = BytesMut::with_capacity(64);
                bet.encode(&mut buf);
                for client in self.ws_clients.lock().await.values() {
                    if let Some(sender) = &client.sender {
                        sender.send(Ok(ws_layer::Msg::binary(buf.clone().freeze().to_vec())));
                    }
                }
                Ok(Response::new(CreateBetReply { id: bet_id }))
            }
            Err(err) => Err(Status::new(Code::from_i32(13), err.to_string())),
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
}
