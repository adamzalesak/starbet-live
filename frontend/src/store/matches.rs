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
    Fetch,
    Update(Match),
}

#[derive(Debug)]
pub enum Action {
    ReceiveResponseUpcoming(anyhow::Result<ListMatchesReply>),
    ReceiveResponseLive(anyhow::Result<ListMatchesReply>),
    ReceiveResponseEnded(anyhow::Result<ListMatchesReply>),
    SetLoading(bool),
    Update(Match),
}

pub struct MatchesStore {
    pub matches_upcoming: Vec<Match>,
    pub matches_live: Vec<Match>,
    pub matches_ended: Vec<Match>,
    pub is_loading: bool,
    pub is_error: bool,
}

impl Store for MatchesStore {
    type Action = Action;
    type Input = MatchesRequest;

    // store's initialization
    fn new() -> Self {
        Self {
            matches_upcoming: Vec::new(),
            matches_live: Vec::new(),
            matches_ended: Vec::new(),
            is_loading: false,
            is_error: false,
        }
    }

    // incoming requests
    fn handle_input(&self, link: AgentLink<StoreWrapper<Self>>, msg: Self::Input) {
        match msg {
            MatchesRequest::Fetch => {
                link.send_message(Action::SetLoading(true));

                link.send_future(async move {
                    let grpc_client = match_service_client::MatchService::new(String::from(
                        "http://127.0.0.1:5430",
                    ));
                    Action::ReceiveResponseUpcoming(
                        grpc_client
                            .list_matches(ListMatchesRequest {
                                game_event_type: GameEventType::Upcoming as i32,
                            })
                            .await,
                    )
                });

                link.send_future(async move {
                    let grpc_client = match_service_client::MatchService::new(String::from(
                        "http://127.0.0.1:5430",
                    ));
                    Action::ReceiveResponseLive(
                        grpc_client
                            .list_matches(ListMatchesRequest {
                                game_event_type: GameEventType::Live as i32,
                            })
                            .await,
                    )
                });

                link.send_future(async move {
                    let grpc_client = match_service_client::MatchService::new(String::from(
                        "http://127.0.0.1:5430",
                    ));
                    Action::ReceiveResponseEnded(
                        grpc_client
                            .list_matches(ListMatchesRequest {
                                game_event_type: GameEventType::Ended as i32,
                            })
                            .await,
                    )
                });
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

            Action::ReceiveResponseUpcoming(Ok(result)) => {
                self.matches_upcoming = result.game_matches;
                self.matches_upcoming
                    .sort_by_key(|m| m.supposed_start_at.clone());
                self.matches_upcoming.reverse();
                self.is_loading = false;
            }
            Action::ReceiveResponseUpcoming(Err(err)) => {
                log::error!("{}", err.to_string());
                self.is_loading = false;
                self.is_error = true;
            }
            Action::ReceiveResponseLive(Ok(result)) => {
                self.matches_live = result.game_matches;
                self.matches_live
                    .sort_by_key(|m| m.supposed_start_at.clone());
                self.matches_live.reverse();
                self.is_loading = false;
            }
            Action::ReceiveResponseLive(Err(err)) => {
                log::error!("{}", err.to_string());
                self.is_loading = false;
                self.is_error = true;
            }
            Action::ReceiveResponseEnded(Ok(result)) => {
                self.matches_ended = result.game_matches;
                self.matches_ended
                    .sort_by_key(|m| m.supposed_start_at.clone());
                self.matches_ended.reverse();
                self.is_loading = false;
            }
            Action::ReceiveResponseEnded(Err(err)) => {
                log::error!("{}", err.to_string());
                self.is_loading = false;
                self.is_error = true;
            }

            Action::Update(match_item) => {
                let match_id = match_item.clone().id;
                self.matches_upcoming = self
                    .matches_upcoming
                    .clone()
                    .into_iter()
                    .filter(|m| m.id != match_id)
                    .collect();
                self.matches_live = self
                    .matches_live
                    .clone()
                    .into_iter()
                    .filter(|m| m.id != match_id)
                    .collect();
                self.matches_ended = self
                    .matches_ended
                    .clone()
                    .into_iter()
                    .filter(|m| m.id != match_id)
                    .collect();

                match match_item.game_event_type {
                    0 => {
                        self.matches_upcoming.push(match_item);
                        self.matches_upcoming
                            .sort_by_key(|m| m.supposed_start_at.clone());
                        self.matches_upcoming.reverse();
                    }
                    1 => {
                        self.matches_live.push(match_item);
                        self.matches_live
                            .sort_by_key(|m| m.supposed_start_at.clone());
                        self.matches_live.reverse();
                    }
                    2 => {
                        self.matches_ended.push(match_item);
                        self.matches_ended
                            .sort_by_key(|m| m.supposed_start_at.clone());
                        self.matches_ended.reverse();
                    }
                    _ => {}
                }
            }
        }
    }
}
