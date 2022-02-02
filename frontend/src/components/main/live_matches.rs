use super::matches_game::MatchesGame;
use crate::components::loading_animation::LoadingAnimation;
use crate::store::{GamesRequest, GamesStore, MatchesRequest, MatchesStore};
use crate::types::grpc_types::game::Game;
use crate::types::grpc_types::game_match::{GameEventType, Match};
use bytes::BytesMut;
use gloo::console::info;
use prost::{DecodeError, Message as ProstMessage};
use wasm_sockets::{self, Message, WebSocketError};
use yew::prelude::*;
use yew_agent::{
    utils::store::{Bridgeable, ReadOnly, StoreWrapper},
    Bridge,
};

pub enum Msg {
    GamesStore(ReadOnly<GamesStore>),
    MatchesStore(ReadOnly<MatchesStore>),
    HandleChange,
}
pub struct Matches {
    filter_ids: Vec<i32>,
    is_loading: bool,
    is_error: bool,

    games_store: Box<dyn Bridge<StoreWrapper<GamesStore>>>,
    matches_store: Box<dyn Bridge<StoreWrapper<MatchesStore>>>,
    games: Vec<Game>,
    live_matches: Vec<Match>,

    matches: Vec<Match>,
}

impl Component for Matches {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            filter_ids: Vec::new(),
            is_loading: false,
            is_error: false,

            games_store: GamesStore::bridge(ctx.link().callback(Msg::GamesStore)),
            matches_store: MatchesStore::bridge(ctx.link().callback(Msg::MatchesStore)),
            games: Vec::new(),
            live_matches: Vec::new(),

            matches: Vec::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GamesStore(state) => {
                let state = state.borrow();
                self.filter_ids = state.filter_ids.clone();
                self.is_loading = state.is_loading.clone();
                self.is_error = state.is_error.clone();
                self.games = state
                    .games
                    .clone()
                    .clone()
                    .into_iter()
                    .filter(|g| !self.filter_ids.contains(&g.id))
                    .collect();

                ctx.link().send_message(Msg::HandleChange);
            }
            Msg::MatchesStore(state) => {
                let state = state.borrow();
                self.live_matches = state.matches_live.clone();

                ctx.link().send_message(Msg::HandleChange);
            }
            Msg::HandleChange => {
                let game_ids: Vec<i32> = self
                    .games
                    .clone()
                    .into_iter()
                    .filter(|g| !self.filter_ids.contains(&g.id))
                    .map(|g| g.id)
                    .collect();
                self.matches = self
                    .live_matches
                    .clone()
                    .into_iter()
                    .filter(|m| game_ids.contains(&m.game_id))
                    .collect()
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <ul class="flex flex-col gap-2 overflow-auto">
            {
                if self.is_loading {
                    html! { <LoadingAnimation color="dark-blue" /> }
                } else if self.is_error {
                    html! { {"error"} }
                } else if self.matches.is_empty() {
                    html! { <div class="bg-blue rounded-md p-1 text-center text-white" >{ "No live matches to show" }</div> }
                } else {
                    self.games.clone().into_iter().map(|game| {
                        let game_id = game.id.clone();
                        html! {
                            <li key={ game.id }>
                                <MatchesGame id={game.id} name={game.name} logo_url={game.logo_url} />
                            </li>
                        }
                    }).collect::<Html>()
                }
            }
            </ul>
        }
    }
}
