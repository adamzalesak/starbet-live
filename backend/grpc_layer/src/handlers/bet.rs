use bytes::BytesMut;
use prost::Message;
use std::convert::*;
use std::sync::Arc;
use tonic::{Code, Request, Response, Status};

use crate::bet::bet_service_server::BetService;
use crate::bet::{
    Bet, CreateBetReply, CreateBetRequest, DeleteBetReply, DeleteBetRequest, ListBetsReply,
    ListBetsRequest,
};

use database_layer::{
    connection::PgPool,
    db_access::{
        bet_and_ticket::{BetAndTicketRepo, PgBetAndTicketRepo},
        repo::Repo,
    },
    db_models::bet::CreateBet,
};
use ws_layer::Clients;

pub struct MyBetService {
    repo: PgBetAndTicketRepo,
    ws_clients: Clients,
}

impl MyBetService {
    pub fn new(pool: &Arc<PgPool>, ws_clients: Clients) -> MyBetService {
        MyBetService {
            repo: PgBetAndTicketRepo::new(pool),
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
        let request = request.into_inner();
        match self.repo.get_bets(request.ticket_id).await {
            Ok(bets) => Ok(Response::new(ListBetsReply {
                bets: bets.iter().map(|bet| Bet::from(bet)).collect(),
            })),
            Err(err) => Err(Status::new(Code::from_i32(13), err.to_string())),
        }
    }

    async fn create_bet(
        &self,
        request: Request<CreateBetRequest>,
    ) -> Result<Response<CreateBetReply>, Status> {
        let request = request.into_inner();
        let create_bet = CreateBet::new(
            request.match_id,
            request.ticket_id,
            request.team_id,
            "", // TODO
        );

        match self.repo.place_a_bet(request.ticket_id, create_bet).await {
            Ok(bet) => {
                let bet = Bet::from(&bet);

                let mut buf = BytesMut::with_capacity(64);
                let _ = bet.encode(&mut buf);
                for client in self.ws_clients.lock().await.values() {
                    if let Some(sender) = &client.sender {
                        let _ =
                            sender.send(Ok(ws_layer::Msg::binary(buf.clone().freeze().to_vec())));
                    }
                }
                Ok(Response::new(CreateBetReply { id: bet.id }))
            }
            Err(err) => Err(Status::new(Code::from_i32(13), err.to_string())),
        }
    }

    async fn delete_bet(
        &self,
        request: Request<DeleteBetRequest>,
    ) -> Result<Response<DeleteBetReply>, Status> {
        let request = request.into_inner();

        match self.repo.discard_a_bet(request.ticket_id, request.id).await {
            Ok(()) => Ok(Response::new(DeleteBetReply {})),
            Err(err) => Err(Status::new(Code::from_i32(13), err.to_string())),
        }
    }
}
