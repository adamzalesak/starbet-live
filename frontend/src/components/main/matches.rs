use super::matches_game::MatchesGame;
use crate::components::loading_animation::LoadingAnimation;
use crate::store::{GamesRequest, GamesStore, MatchesRequest, MatchesStore};
use crate::types::grpc_types::game::Game;
use crate::types::grpc_types::game_match::Match;
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
    FetchMatches,
    ReceiveMatchUpdate(Result<Match, DecodeError>),
}
pub struct Matches {
    games: Vec<Game>,
    filter_ids: Vec<i32>,
    is_loading: bool,
    is_error: bool,

    games_store: Box<dyn Bridge<StoreWrapper<GamesStore>>>,
    matches_store: Box<dyn Bridge<StoreWrapper<MatchesStore>>>,

    ws_client: wasm_sockets::EventClient,
}

impl Component for Matches {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let mut client = wasm_sockets::EventClient::new("ws://127.0.0.1:50052/match").unwrap();

        let callback = ctx
            .link()
            .callback(|match_item: Result<Match, DecodeError>| Msg::ReceiveMatchUpdate(match_item));

        client.set_on_message(Some(Box::new(
            move |_: &wasm_sockets::EventClient, message: wasm_sockets::Message| {
                if let Message::Binary(data) = message {
                    let mut buf = BytesMut::with_capacity(64);
                    buf.extend_from_slice(&data);
                    callback.emit(Match::decode(buf));
                };
            },
        )));

        Self {
            games: Vec::new(),
            filter_ids: Vec::new(),
            is_loading: false,
            is_error: false,

            games_store: GamesStore::bridge(ctx.link().callback(Msg::GamesStore)),
            matches_store: MatchesStore::bridge(ctx.link().callback(Msg::MatchesStore)),

            ws_client: client,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GamesStore(state) => {
                let state = state.borrow();
                self.games = state.games.clone();
                self.filter_ids = state.filter_ids.clone();
                self.is_loading = state.is_loading.clone();
                self.is_error = state.is_error.clone();

                ctx.link().send_message(Msg::FetchMatches);
            }
            Msg::MatchesStore(state) => {
                let state = state.borrow();
            }
            Msg::FetchMatches => {
                self.matches_store.send(MatchesRequest::Reset);
                self.games.clone().into_iter().for_each(|game| {
                    if !self.filter_ids.contains(&game.id) {
                        self.matches_store.send(MatchesRequest::Fetch(game.id));
                    }
                });
            }
            Msg::ReceiveMatchUpdate(Ok(match_item)) => {
                if self
                    .games
                    .clone()
                    .into_iter()
                    .map(|game| game.id)
                    .collect::<Vec<i32>>()
                    .contains(&match_item.game_id)
                {
                    self.matches_store.send(MatchesRequest::Update(match_item));
                }
            }
            Msg::ReceiveMatchUpdate(Err(err)) => {
                log::warn!("WebSocket message decode error");
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            if self.is_loading {
                <LoadingAnimation color="dark-blue" />
            } else if self.is_error {
                <h1>{"error"}</h1>
            } else {
                <ul class="flex flex-col gap-2 overflow-auto">
                {
                    if self.games.is_empty() {
                        html! { <div class="bg-blue rounded-md p-1 text-center text-white" >{ "No live matches to show" }</div> }
                    } else {
                        self.games.clone().into_iter().map(|game| {
                            let disabled = self.filter_ids.contains(&game.id);
                            let game_id = game.id.clone();
                            html! {
                                if !disabled {
                                    <li key={ game.id }>
                                        <MatchesGame id={game.id} name={game.name} logo_url={game.logo_url} />
                                    </li>
                                }
                            }
                        }).collect::<Html>()
                    }
                }
                </ul>
            }
        }
    }
}
