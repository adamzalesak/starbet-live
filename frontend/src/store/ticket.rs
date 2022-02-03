use crate::types::grpc_types::bet::{
    bet_service_client, Bet, CreateBetReply, CreateBetRequest, DeleteBetReply, DeleteBetRequest,
};
use crate::types::grpc_types::ticket::{
    ticket_service_client, GetCurrentTicketReply, GetCurrentTicketRequest, SubmitTicketReply,
    SubmitTicketRequest,
};
use anyhow;
use gloo::console::{error, info};
use std::collections::HashMap;
use yew_agent::utils::store::{Store, StoreWrapper};
use yew_agent::AgentLink;

#[derive(Debug)]
pub enum TicketRequest {
    SetUserId(i32),
    LoadTicket,
    CreateBet(i32, i32),
    DeleteBet(i32),
    SubmitTicket,
    ChangeTicketValue(f32),
}

#[derive(Debug)]
pub enum Action {
    SetUserId(i32),
    LoadTicket(anyhow::Result<GetCurrentTicketReply>),
    SetTicketValue(f32),
    CreateBetReceiveResponse(anyhow::Result<CreateBetReply>),
    DeleteBetReceiveResponse(anyhow::Result<DeleteBetReply>),
    SubmitTicketReceiveResponse(anyhow::Result<SubmitTicketReply>),
}

pub struct TicketStore {
    pub id: i32,
    pub user_id: i32,
    pub bets: Vec<Bet>,
    pub ticket_value: f32,
    pub rate: f32,
}

impl Store for TicketStore {
    type Action = Action;
    type Input = TicketRequest;

    // store's initialization
    fn new() -> Self {
        Self {
            id: 0,
            user_id: 0,
            bets: Vec::new(),
            ticket_value: 1.0,
            rate: 1.0,
        }
    }

    // incoming requests
    fn handle_input(&self, link: AgentLink<StoreWrapper<Self>>, msg: Self::Input) {
        match msg {
            TicketRequest::SetUserId(id) => {
                link.send_message(Action::SetUserId(id));
            }
            TicketRequest::LoadTicket => {
                let user_id = self.user_id.clone();
                let grpc_client = ticket_service_client::TicketService::new(String::from(
                    "http://127.0.0.1:5430",
                ));
                link.send_future(async move {
                    Action::LoadTicket(
                        grpc_client
                            .get_current_ticket(GetCurrentTicketRequest { user_id })
                            .await,
                    )
                })
            }
            TicketRequest::CreateBet(match_id, team_id) => {
                if self.id == 0 {
                    return;
                };
                let ticket_id = self.id.clone();
                let grpc_client =
                    bet_service_client::BetService::new(String::from("http://127.0.0.1:5430"));
                link.send_future(async move {
                    Action::CreateBetReceiveResponse(
                        grpc_client
                            .create_bet(CreateBetRequest {
                                ticket_id: ticket_id,
                                match_id: match_id.clone(),
                                team_id: team_id.clone(),
                            })
                            .await,
                    )
                })
            }
            TicketRequest::DeleteBet(id) => {
                let ticket_id = self.id.clone();
                let grpc_client =
                    bet_service_client::BetService::new(String::from("http://127.0.0.1:5430"));
                link.send_future(async move {
                    Action::DeleteBetReceiveResponse(
                        grpc_client
                            .delete_bet(DeleteBetRequest { id, ticket_id })
                            .await,
                    )
                });
                let ticket_id = self.id.clone();
                let grpc_client =
                    bet_service_client::BetService::new(String::from("http://127.0.0.1:5430"));

                // reload ticket
                let user_id = self.user_id.clone();
                let grpc_client = ticket_service_client::TicketService::new(String::from(
                    "http://127.0.0.1:5430",
                ));
                link.send_future(async move {
                    Action::LoadTicket(
                        grpc_client
                            .get_current_ticket(GetCurrentTicketRequest { user_id })
                            .await,
                    )
                });
            }
            // TODO
            TicketRequest::SubmitTicket => {
                let ticket_id = self.id.clone();
                let price_paid = self.ticket_value.clone();
                let grpc_client = ticket_service_client::TicketService::new(String::from(
                    "http://127.0.0.1:5430",
                ));
                link.send_future(async move {
                    Action::SubmitTicketReceiveResponse(
                        grpc_client
                            .submit_ticket(SubmitTicketRequest {
                                ticket_id,
                                price_paid,
                            })
                            .await,
                    )
                });

                // reload ticket
                let user_id = self.user_id.clone();
                let grpc_client = ticket_service_client::TicketService::new(String::from(
                    "http://127.0.0.1:5430",
                ));
                link.send_future(async move {
                    Action::LoadTicket(
                        grpc_client
                            .get_current_ticket(GetCurrentTicketRequest { user_id })
                            .await,
                    )
                });
            }
            TicketRequest::ChangeTicketValue(value) => {
                link.send_message(Action::SetTicketValue(value));
            }
        }
    }

    // store's operations
    fn reduce(&mut self, msg: Self::Action) {
        match msg {
            Action::SetUserId(id) => {
                self.user_id = id;
            }
            Action::SetTicketValue(value) => {
                self.ticket_value = value;
            }
            Action::LoadTicket(Ok(ticket_reply)) => {
                self.id = ticket_reply.ticket_id;
                self.bets = ticket_reply.bets;
            }
            Action::LoadTicket(Err(_)) => {
                // todo handle error
            }
            Action::CreateBetReceiveResponse(Ok(_)) => {
                info!("bet created");
            }
            Action::CreateBetReceiveResponse(Err(err)) => {
                error!("error create bet", err.to_string());
            }
            Action::DeleteBetReceiveResponse(Ok(_)) => {
                info!("bet deleted");
            }
            Action::DeleteBetReceiveResponse(Err(err)) => {
                error!("error delete bet", err.to_string());
            }
            Action::SubmitTicketReceiveResponse(Ok(_)) => {
                info!("ticket submited");
            }
            Action::SubmitTicketReceiveResponse(Err(err)) => {
                error!("error submit ticket", err.to_string());
            }
        }
    }
}
