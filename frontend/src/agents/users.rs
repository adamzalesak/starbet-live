use std::collections::HashMap;
use yew_agent::utils::store::{Store, StoreWrapper};
use yew_agent::AgentLink;

use crate::types::tickets::BetInfo;

type Id = u32;

#[derive(Debug)]
pub enum TicketRequest {
    // AddBet(BetInfo),
    // DeleteBet(Id),
    // SubmitTicket,
    // ChangeTicketValue(f32),
}

#[derive(Debug)]
pub enum Action {
    SetUser(),
}

pub struct TicketStore {
    pub id: Id, // user's id
    pub first_name: String,
    pub last_name: String,
    pub token: String, //jwt token loaded from local storage 
    pub current_balance: String,
}

impl Store for TicketStore {
    type Action = Action;
    type Input = TicketRequest;

    // store's initialization
    fn new() -> Self {
        Self {
            id: 0,
            first_name: String::new(),
            last_name: String::new(),
            token: String::new(), //jwt token, but stored in local storage. 
            current_balance: String::new(),
        }
    }

    // incoming requests
    fn handle_input(&self, link: AgentLink<StoreWrapper<Self>>, msg: Self::Input) {
        
    }

    // store's operations
    fn reduce(&mut self, msg: Self::Action) {
        // match msg {
        //     Action::SetBet(id, bet) => {
        //         self.bets.insert(id, bet);
        //     }
        //     Action::RemoveBet(id) => {
        //         self.bets.remove(&id);
        //     }
        //     Action::SetTicketValue(value) => {
        //         self.ticket_value = value;
        //     }
        // }
    }
}
