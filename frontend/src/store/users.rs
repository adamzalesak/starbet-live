use crate::services::{get_token, set_token};
use crate::types::grpc_types::user::{
    user_service_client, Address, AuthUserReply, AuthUserRequest, GetUserReply, GetUserRequest,
    User,
};
use log::{error, info};
use yew_agent::utils::store::{Store, StoreWrapper};
use yew_agent::AgentLink;

#[derive(Debug)]
pub enum UserRequest {
    Login(Option<User>),
    Logout,
    InitializeUser,
}

#[derive(Debug)]
pub enum Action {
    SetUser(User),
    RemoveUser,
    ReceiveResponse(anyhow::Result<GetUserReply>),
}

pub struct UserStore {
    pub user: Option<User>,
}

impl Store for UserStore {
    type Action = Action;
    type Input = UserRequest;

    // store's initialization
    fn new() -> Self {
        Self { user: None }
    }

    // incoming requests
    fn handle_input(&self, link: AgentLink<StoreWrapper<Self>>, msg: Self::Input) {
        match msg {
            UserRequest::Login(user) => match user {
                Some(value) => link.send_message(Action::SetUser(value)),
                None => error!("Couldn't set user"),
            },
            UserRequest::InitializeUser => {
                info!("Retrieving token from local storage!");
                match get_token() {
                    Some(token) => {
                        info!("token -> {}", token);

                        // get identity and create user
                        let grpc_client = user_service_client::UserService::new(String::from(
                            "http://127.0.0.1:5430",
                        ));
                        link.send_future(async move {
                            Action::ReceiveResponse(
                                grpc_client.get_user(GetUserRequest { id: 2 }).await,
                            )
                        });
                    }
                    None => {}
                }
            }
            UserRequest::Logout => link.send_message(Action::RemoveUser),
        }
    }

    // store's operations
    fn reduce(&mut self, msg: Self::Action) {
        match msg {
            Action::SetUser(user_data) => {
                self.user = Some(user_data.clone());
                set_token(Some(user_data.id.to_string()));
            }
            Action::ReceiveResponse(Ok(data)) => match data.user {
                Some(user_data) => {
                    self.user = Some(user_data.clone());
                    set_token(Some(user_data.id.to_string()));
                }
                None => error!("Couldn't set user"),
            },
            Action::ReceiveResponse(Err(err)) => {
                error!("{}", err.to_string());
            }
            Action::RemoveUser => {
                self.user = None;
                set_token(None);
            }
        }
    }
}
