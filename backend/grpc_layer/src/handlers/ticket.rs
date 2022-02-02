use bytes::BytesMut;
use prost::Message;
use std::sync::Arc;
use tonic::{Code, Request, Response, Status};

use crate::bet::Bet;
use crate::game_match::{GameEventType, Match};
use crate::ticket::ticket_service_server::TicketService;
use crate::ticket::{
    GetCurrentTicketReply, GetCurrentTicketRequest, ListTicketsReply, ListTicketsRequest,
    SubmitTicketReply, SubmitTicketRequest, Ticket,
};

use database_layer::{
    connection::PgPool,
    db_access::{
        bet_and_ticket::{BetAndTicketRepo, PgBetAndTicketRepo},
        game_match::{MatchRepo, PgMatchRepo},
        repo::Repo,
        submitted_bet_and_ticket::{PgSubmittedBetAndTicketRepo, SubmittedBetAndTicketRepo},
    },
    db_models::{game_match_event::GameMatchEventType, ticket::ObtainedTicket},
};
use ws_layer::Clients;

pub struct MyTicketService {
    repo: PgBetAndTicketRepo,
    submitted_repo: PgSubmittedBetAndTicketRepo,
    match_repo: PgMatchRepo,
    ws_clients: Clients,
}

impl MyTicketService {
    pub fn new(pool: &Arc<PgPool>, ws_clients: Clients) -> MyTicketService {
        MyTicketService {
            repo: PgBetAndTicketRepo::new(pool),
            submitted_repo: PgSubmittedBetAndTicketRepo::new(pool),
            match_repo: PgMatchRepo::new(pool),
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
        match self.repo.get_user_current_ticket(request.user_id).await {
            Ok(ObtainedTicket::StillValid(ticket))
            | Ok(ObtainedTicket::NoTicketFound(ticket))
            | Ok(ObtainedTicket::NewAfterInvalid(ticket)) => {
                match self.repo.get_bets(ticket.id).await {
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

        match self.submitted_repo.get_all(request.user_id).await {
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
            .repo
            .submit_ticket(request.ticket_id, request.price_paid.into())
            .await
        {
            Ok(submitted_ticket_id) => {
                match self.submitted_repo.get_bets(submitted_ticket_id).await {
                    Ok(submitted_bets) => {
                        for bet in submitted_bets {
                            /*
                            let ratios  = self.match_repo.get_ratios();
                            1. 1.1
                            2. 0.95
                            self.match_repo.set_ratios(ratios);
                            */
                            let game_match_info;
                            match self.match_repo.get_show_info(bet.game_match_id).await {
                                Ok(match_info) => game_match_info = match_info,
                                Err(err) => {
                                    return Err(Status::new(Code::from_i32(13), err.to_string()))
                                }
                            }
                            let (game_match, game_event_type) = game_match_info;
                            let mut winner_id = None;
                            let grpc_event_type = match game_event_type.extract_event().unwrap() {
                                GameMatchEventType::Upcoming => GameEventType::Upcoming,
                                GameMatchEventType::Live(_) => GameEventType::Live,
                                GameMatchEventType::Ended(id) => {
                                    winner_id = Some(id);
                                    GameEventType::Ended
                                }
                                _ => GameEventType::Upcoming,
                            };
                            let game_match = Match {
                                id: game_match.id,
                                game_id: game_match.game_id,
                                team_one: None,
                                team_two: None,
                                team_one_ratio: game_match.team_one_ratio,
                                team_two_ratio: game_match.team_two_ratio,
                                supposed_start_at: game_match.supposed_start_at,
                                state: game_match.state,
                                game_event_type: grpc_event_type.into(),
                                winner_id: winner_id,
                            };

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
                        Ok(Response::new(SubmitTicketReply {}))
                    }
                    Err(err) => Err(Status::new(Code::from_i32(13), err.to_string())),
                }
            }
            Err(err) => Err(Status::new(Code::from_i32(13), err.to_string())),
        }
    }
}
