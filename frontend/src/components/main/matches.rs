use crate::components::loading_animation::LoadingAnimation;
use crate::store::{
    games::game::Game, matches::game_match::Match, GamesRequest, GamesStore, MatchesRequest,
    MatchesStore,
};
use yew::prelude::*;
use yew_agent::{
    utils::store::{Bridgeable, ReadOnly, StoreWrapper},
    Bridge,
};

pub mod game {
    include!(concat!(env!("OUT_DIR"), concat!("/game.rs")));
}

use super::matches_game::MatchesGame;

pub enum Msg {
    GamesStore(ReadOnly<GamesStore>),
    MatchesStore(ReadOnly<MatchesStore>),
    FetchMatches,
}
pub struct Matches {
    games: Vec<Game>,
    filter_ids: Vec<i32>,
    is_loading: bool,
    is_error: bool,
    games_store: Box<dyn Bridge<StoreWrapper<GamesStore>>>,
    matches_store: Box<dyn Bridge<StoreWrapper<MatchesStore>>>,
}

impl Component for Matches {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            games: Vec::new(),
            filter_ids: Vec::new(),
            is_loading: false,
            is_error: false,
            games_store: GamesStore::bridge(ctx.link().callback(Msg::GamesStore)),
            matches_store: MatchesStore::bridge(ctx.link().callback(Msg::MatchesStore)),
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
                </ul>
            }
        }
    }
}
