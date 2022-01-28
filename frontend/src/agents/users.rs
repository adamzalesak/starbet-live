use yew_agent::utils::store::{Store, StoreWrapper};
use yew_agent::AgentLink;

use crate::types::users::UserStorage;

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

pub struct UserStore {
    pub user: UserStorage,
}

impl Store for UserStore {
    type Action = Action;
    type Input = TicketRequest;

    // store's initialization
    fn new() -> Self {
        Self {
            user: UserStorage::new()
        }
    }

    // incoming requests
    fn handle_input(&self, _link: AgentLink<StoreWrapper<Self>>, _msg: Self::Input) {
        
    }

    // store's operations
    fn reduce(&mut self, _msg: Self::Action) {
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
