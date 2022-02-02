use anyhow;
use std::collections::HashMap;
use yew_agent::utils::store::{Store, StoreWrapper};
use yew_agent::AgentLink;

use crate::types::tickets::BetInfo;

pub mod game {
    include!(concat!(env!("OUT_DIR"), concat!("/game.rs")));
}
use game::{game_service_client, Game, ListGamesReply, ListGamesRequest};

#[derive(Debug)]
pub enum GamesRequest {
    Fetch,
    FilterAdd(i32),
    FilterRemove(i32),
}

#[derive(Debug)]
pub enum Action {
    FilterAdd(i32),
    FilterRemove(i32),
    ReceiveResponse(anyhow::Result<ListGamesReply>),
    SetLoading(bool),
}

pub struct GamesStore {
    pub games: Vec<Game>,
    pub filter_ids: Vec<i32>,
    pub is_loading: bool,
    pub is_error: bool,
}

impl Store for GamesStore {
    type Action = Action;
    type Input = GamesRequest;

    // store's initialization
    fn new() -> Self {
        Self {
            games: Vec::new(),
            filter_ids: Vec::new(),
            is_loading: false,
            is_error: false,
        }
    }

    // incoming requests
    fn handle_input(&self, link: AgentLink<StoreWrapper<Self>>, msg: Self::Input) {
        match msg {
            GamesRequest::Fetch => {
                link.send_message(Action::SetLoading(true));

                let grpc_client =
                    game_service_client::GameService::new(String::from("http://127.0.0.1:5430"));
                link.send_future(async move {
                    Action::ReceiveResponse(grpc_client.list_games(ListGamesRequest {}).await)
                });
            }
            GamesRequest::FilterAdd(id) => {
                link.send_message(Action::FilterAdd(id));
            }
            GamesRequest::FilterRemove(id) => {
                link.send_message(Action::FilterRemove(id));
            }
        }
    }

    // store's operations
    fn reduce(&mut self, msg: Self::Action) {
        match msg {
            Action::SetLoading(value) => {
                self.is_loading = value;
            }
            Action::ReceiveResponse(Ok(result)) => {
                self.games = result.games;
                self.is_loading = false;
            }
            Action::ReceiveResponse(Err(_)) => {
                self.games = Vec::new();
                self.is_loading = false;
                self.is_error = true;
            }
            Action::FilterAdd(id) => self.filter_ids.push(id),
            Action::FilterRemove(id) => {
                self.filter_ids = self
                    .filter_ids
                    .clone()
                    .into_iter()
                    .filter(|x| *x != id)
                    .collect()
            }
        }
    }
}
