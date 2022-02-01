use gloo::console::info;
use std::collections::HashMap;
use yew_agent::utils::store::{Store, StoreWrapper};
use yew_agent::AgentLink;

use crate::types::tickets::BetInfo;

pub mod game_match {
    include!(concat!(env!("OUT_DIR"), concat!("/game_match.rs")));
}
pub mod team {
    include!(concat!(env!("OUT_DIR"), concat!("/team.rs")));
}
use game_match::{
    match_service_client, GameEventType, ListMatchesReply, ListMatchesRequest, Match,
};

#[derive(Debug)]
pub enum MatchesRequest {
    Fetch(i32),
    Reset,
}

#[derive(Debug)]
pub enum Action {
    ReceiveResponse(Result<ListMatchesReply, Box<dyn std::error::Error>>),
    SetLoading(bool),
    Reset,
}

pub struct MatchesStore {
    pub matches: Vec<Match>,
    pub is_loading: bool,
    pub is_error: bool,
}

impl Store for MatchesStore {
    type Action = Action;
    type Input = MatchesRequest;

    // store's initialization
    fn new() -> Self {
        Self {
            matches: Vec::new(),
            is_loading: false,
            is_error: false,
        }
    }

    // incoming requests
    fn handle_input(&self, link: AgentLink<StoreWrapper<Self>>, msg: Self::Input) {
        match msg {
            MatchesRequest::Fetch(game_id) => {
                link.send_message(Action::SetLoading(true));

                let grpc_client =
                    match_service_client::MatchService::new(String::from("http://127.0.0.1:5430"));
                info!("MatchesRequest::Fetch", game_id);
                // todo uncomment after bug fixed
                // link.send_future(async move {
                //     Action::ReceiveResponse(
                //         grpc_client
                //             .list_matches(ListMatchesRequest {
                //                 game_id: game_id,
                //                 game_event_type: GameEventType::Live as i32,
                //             })
                //             .await,
                //     )
                // });
            }
            MatchesRequest::Reset => {
                link.send_message(Action::Reset);
            }
        }
    }

    // store's operations
    fn reduce(&mut self, msg: Self::Action) {
        match msg {
            Action::SetLoading(value) => {
                self.is_loading = true;
            }
            Action::ReceiveResponse(Ok(result)) => {
                self.matches.extend(result.game_matches);
                self.is_loading = false;
            }
            Action::ReceiveResponse(Err(_)) => {
                self.is_loading = false;
                self.is_error = true;
            }
            Action::Reset => {
                self.matches = Vec::new();
            }
        }
    }
}
