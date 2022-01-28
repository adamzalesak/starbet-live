use std::collections::HashMap;
use yew_agent::utils::store::{Store, StoreWrapper};
use yew_agent::AgentLink;

use crate::types::tickets::BetInfo;

type Id = u32;

#[derive(Debug)]
pub enum TicketRequest {
    AddBet(BetInfo),
    DeleteBet(Id),
    SubmitTicket,
    ChangeTicketValue(f32),
}

#[derive(Debug)]
pub enum Action {
    SetBet(Id, BetInfo),
    RemoveBet(Id),
    SetTicketValue(f32),
}

pub struct TicketStore {
    pub id: Id,
    pub bets: HashMap<Id, BetInfo>,
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
            bets: HashMap::new(),
            ticket_value: 1.0,
            rate: 1.0,
        }
    }

    // incoming requests
    fn handle_input(&self, link: AgentLink<StoreWrapper<Self>>, msg: Self::Input) {
        match msg {
            TicketRequest::AddBet(val) => {
                link.send_message(Action::SetBet(val.id, val));
            }
            TicketRequest::DeleteBet(id) => {
                link.send_message(Action::RemoveBet(id));
            }
            TicketRequest::SubmitTicket => {}
            TicketRequest::ChangeTicketValue(value) => {
                link.send_message(Action::SetTicketValue(value));
            }
        }
    }

    // store's operations
    fn reduce(&mut self, msg: Self::Action) {
        match msg {
            Action::SetBet(id, bet) => {
                self.bets.insert(id, bet);
            }
            Action::RemoveBet(id) => {
                self.bets.remove(&id);
            }
            Action::SetTicketValue(value) => {
                self.ticket_value = value;
            }
        }
    }
}
