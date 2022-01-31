use crate::services::{get_token, set_token};
use crate::types::users::UserInfo;
use log::info;
use yew_agent::utils::store::{Store, StoreWrapper};
use yew_agent::AgentLink;

#[derive(Debug)]
pub enum UserRequest {
    Login(UserInfo),
    // AddBet(BetInfo),
    // DeleteBet(Id),
    // SubmitTicket,
    // ChangeTicketValue(f32),
}

#[derive(Debug)]
pub enum Action {
    SetUser(UserInfo),
}

pub struct UserStore {
    pub logged_in: bool,
    pub user: UserInfo,
}

fn retrieve_data() -> Option<UserInfo> {
    set_token(Some("Jebakova rit".to_string()));
    info!("Retrieving token from local storage!");
    match get_token() {
        Some(token) => {
            info!("token -> {}", token);

            // get identity and create UserInfo

            // temporary 
            let mut temp = UserInfo::new();
            temp.first_name = "Jozef".to_string();
            temp.last_name = "Kubani".to_string();
            temp.current_balance = "21231.21".to_string();
            temp.token = "3o24fh9834hf028hfh973hfwhef938hfodsnaimjw2jedowh".to_string();
            temp.id = 0;

            Some(temp)
        }
        None => {
            info!("No token");
            None
        }
    }
}

impl Store for UserStore {
    type Action = Action;
    type Input = UserRequest;

    // store's initialization
    fn new() -> Self {
        let initial_state = match retrieve_data() {
            Some(val) => val,
            None => UserInfo::new(),
        };
        Self {
            logged_in: false,
            user: initial_state,
        }
    }

    // incoming requests
    fn handle_input(&self, link: AgentLink<StoreWrapper<Self>>, msg: Self::Input) {
        match msg {
            UserRequest::Login(user_data) => {
                link.send_message(Action::SetUser(user_data));
            }
        }
    }

    // store's operations
    fn reduce(&mut self, msg: Self::Action) {
        match msg {
            Action::SetUser(user_data) => {
                self.logged_in = true;
                self.user = user_data
            }
        }
    }
}
