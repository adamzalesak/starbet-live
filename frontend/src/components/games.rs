use crate::components::loading_animation::LoadingAnimation;
use crate::store::{GamesRequest, GamesStore};
use crate::types::grpc_types::game::{game_service_client, Game, ListGamesReply, ListGamesRequest};
use log::info;
use yew::prelude::*;
use yew_agent::{
    utils::store::{Bridgeable, ReadOnly, StoreWrapper},
    Bridge,
};

pub enum Msg {
    GamesStore(ReadOnly<GamesStore>),
    Fetch,
    FilterAdd(i32),
    FilterRemove(i32),
}

pub struct Games {
    games: Vec<Game>,
    filter_ids: Vec<i32>,
    is_loading: bool,
    is_error: bool,
    games_store: Box<dyn Bridge<StoreWrapper<GamesStore>>>,
}

impl Component for Games {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::Fetch);
        Self {
            games: Vec::new(),
            filter_ids: Vec::new(),
            is_loading: false,
            is_error: false,
            games_store: GamesStore::bridge(ctx.link().callback(Msg::GamesStore)),
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
            }
            Msg::Fetch => {
                self.games_store.send(GamesRequest::Fetch);
            }
            Msg::FilterAdd(id) => {
                self.games_store.send(GamesRequest::FilterAdd(id));
            }
            Msg::FilterRemove(id) => {
                self.games_store.send(GamesRequest::FilterRemove(id));
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
         <div class="bg-dark-blue text-white flex-auto rounded-md p-2 text-center h-4/5 max-h-full">
            <div class="h-full flex flex-col">
                <div class="font-bold mb-2">{"Games"}</div>

                if self.is_loading {
                    <LoadingAnimation color="white" />
                }
                else if self.is_error {
                    <h1>{"error"}</h1>
                }
                else {
                    <ul class="flex flex-col gap-1.5 overflow-auto">
                        {
                            self.games.clone().into_iter().map(|game| {
                                let disabled = self.filter_ids.contains(&game.id);
                                let game_id = game.id.clone();
                                html! {
                                    <li
                                        key={ game.id }
                                        onclick={ if disabled { ctx.link().callback(move |_|Msg::FilterRemove(game_id)) } else { ctx.link().callback(move |_|Msg::FilterAdd(game_id)) } }
                                        class={format!("flex flex-row gap-2 rounded-md p-1 text-black text-left font-bold cursor-pointer {}", if disabled {"bg-gray-400"} else {"bg-white"})}>
                                        if game.logo_url != "" {
                                            <div class="w-6 h-6 my-auto">
                                                <img src={game.logo_url.clone()} class="w-full" alt={game.name.clone()} />
                                            </div>
                                        }
                                        <div>{ game.name.clone() }</div>
                                    </li>
                                }
                            }).collect::<Html>()
                        }
                    </ul>
                }
                </div>

        </div>
        }
    }
}
