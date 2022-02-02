use crate::types::grpc_types::game_match::{
    match_service_client, GameEventType, ListMatchesReply, ListMatchesRequest, Match,
};
use crate::types::tickets::BetInfo;
use anyhow;
use gloo::console::info;
use std::collections::HashMap;
use yew_agent::utils::store::{Store, StoreWrapper};
use yew_agent::AgentLink;

#[derive(Debug)]
pub enum MatchesRequest {
    Fetch(i32),
    Reset,
    Update(Match),
}

#[derive(Debug)]
pub enum Action {
    ReceiveResponse(anyhow::Result<ListMatchesReply>),
    SetLoading(bool),
    Reset,
    Update(Match),
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
                link.send_future(async move {
                    Action::ReceiveResponse(
                        grpc_client
                            .list_matches(ListMatchesRequest {
                                game_id: game_id,
                                game_event_type: 1,
                            })
                            .await,
                    )
                });
            }
            MatchesRequest::Reset => {
                link.send_message(Action::Reset);
            }
            MatchesRequest::Update(match_item) => {
                link.send_message(Action::Update(match_item));
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
                self.matches.sort_by_key(|match_item| match_item.id);
                self.is_loading = false;
            }
            Action::ReceiveResponse(Err(err)) => {
                log::warn!("cringe? {}", err.to_string());
                self.is_loading = false;
                self.is_error = true;
            }
            Action::Reset => {
                self.matches = Vec::new();
            }
            Action::Update(match_item) => {
                self.matches = self
                    .matches
                    .clone()
                    .into_iter()
                    .filter(|m| m.id != match_item.id)
                    .collect::<Vec<Match>>();
                self.matches.push(match_item);
                self.matches.sort_by_key(|match_item| match_item.id);
            }
        }
    }
}
