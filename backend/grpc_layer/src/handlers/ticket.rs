use bytes::BytesMut;
use prost::Message;
use std::sync::Arc;
use tonic::{Code, Request, Response, Status};

use crate::bet::Bet;
use crate::repos::Repos;
use crate::ticket::ticket_service_server::TicketService;
use crate::ticket::{
    GetCurrentTicketReply, GetCurrentTicketRequest, ListTicketsReply, ListTicketsRequest,
    SubmitTicketReply, SubmitTicketRequest, Ticket,
};

use database_layer::{
    connection::PgPool,
    db_access::{
        bet_and_ticket::BetAndTicketRepo, submitted_bet_and_ticket::SubmittedBetAndTicketRepo,
    },
    db_models::ticket::ObtainedTicket,
};
use ws_layer::Clients;

pub struct MyTicketService {
    repos: Repos,
    ws_clients: Clients,
}

impl MyTicketService {
    pub fn new(pool: &Arc<PgPool>, ws_clients: Clients) -> MyTicketService {
        MyTicketService {
            repos: Repos::new(pool),
            ws_clients: ws_clients,
        }
    }
}

#[tonic::async_trait]
impl TicketService for MyTicketService {
    async fn get_current_ticket(
        &self,
        request: Request<GetCurrentTicketRequest>,
    ) -> Result<Response<GetCurrentTicketReply>, Status> {
        let request = request.into_inner();
        match self
            .repos
            .bet_ticket
            .get_user_current_ticket(request.user_id)
            .await
        {
            Ok(ObtainedTicket::StillValid(ticket))
            | Ok(ObtainedTicket::NoTicketFound(ticket))
            | Ok(ObtainedTicket::NewAfterInvalid(ticket)) => {
                match self.repos.bet_ticket.get_bets(ticket.id).await {
                    Ok(bets) => Ok(Response::new(GetCurrentTicketReply {
                        ticket_id: ticket.id,
                        bets: bets.iter().map(|bet| Bet::from(bet)).collect(),
                    })),
                    Err(err) => Err(Status::new(Code::from_i32(13), err.to_string())),
                }
            }
            Err(err) => Err(Status::new(Code::from_i32(13), err.to_string())),
        }
    }

    async fn list_tickets(
        &self,
        request: Request<ListTicketsRequest>,
    ) -> Result<Response<ListTicketsReply>, Status> {
        let request = request.into_inner();

        match self.repos.sub_bet_ticket.get_all(request.user_id).await {
            Ok(tickets_bets) => Ok(Response::new(ListTicketsReply {
                tickets: tickets_bets
                    .iter()
                    .map(|(ticket, bets)| Ticket {
                        id: ticket.id,
                        submitted_at: ticket.submitted_at.clone(),
                        price_paid: ticket.price_paid.clone(),
                        total_ratio: ticket.total_ratio.clone(),
                        won: ticket.won,
                        bets: bets
                            .iter()
                            .map(|bet| Bet {
                                id: bet.id,
                                ticket_id: bet.submitted_ticket_id,
                                match_id: bet.game_match_id,
                                team_id: bet.team_id,
                            })
                            .collect(),
                    })
                    .collect(),
            })),
            Err(err) => Err(Status::new(Code::from_i32(13), err.to_string())),
        }
    }

    async fn submit_ticket(
        &self,
        request: Request<SubmitTicketRequest>,
    ) -> Result<Response<SubmitTicketReply>, Status> {
        let request = request.into_inner();
        match self
            .repos
            .bet_ticket
            .submit_ticket(request.ticket_id, request.price_paid.into())
            .await
        {
            Ok(submitted_ticket_id) => {
                match self
                    .repos
                    .sub_bet_ticket
                    .get_bets(submitted_ticket_id)
                    .await
                {
                    Ok(submitted_bets) => {
                        for bet in submitted_bets {
                            match self
                                .repos
                                .change_ratios(bet.game_match_id, bet.team_id)
                                .await
                            {
                                Ok(_) => {}
                                Err(err) => {
                                    return Err(Status::new(Code::from_i32(13), err.to_string()))
                                }
                            }
                            match self.repos.get_filled_match(bet.game_match_id).await {
                                Ok(game_match) => {
                                    let mut buf = BytesMut::with_capacity(64);
                                    let _ = game_match.encode(&mut buf);
                                    for client in self.ws_clients.lock().await.values() {
                                        if let Some(sender) = &client.sender {
                                            let _ = sender.send(Ok(ws_layer::Msg::binary(
                                                buf.clone().freeze().to_vec(),
                                            )));
                                        }
                                    }
                                }
                                Err(err) => {
                                    return Err(Status::new(Code::from_i32(13), err.to_string()))
                                }
                            }
                        }
                        Ok(Response::new(SubmitTicketReply {}))
                    }
                    Err(err) => Err(Status::new(Code::from_i32(13), err.to_string())),
                }
            }
            Err(err) => Err(Status::new(Code::from_i32(13), err.to_string())),
        }
    }
}
