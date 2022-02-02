use crate::components::loading_animation::LoadingAnimation;
use crate::store::{matches::game_match::Match, MatchesRequest, MatchesStore};
use anyhow;
use yew::prelude::*;

use super::match_item::MatchItem;

pub mod game_match {
    include!(concat!(env!("OUT_DIR"), concat!("/game_match.rs")));
}
pub mod team {
    include!(concat!(env!("OUT_DIR"), concat!("/team.rs")));
}
use game_match::{match_service_client, GameEventType, ListMatchesReply, ListMatchesRequest};
use team::Team;
use yew_agent::{
    utils::store::{Bridgeable, ReadOnly, StoreWrapper},
    Bridge,
};

pub enum Msg {
    MatchesStore(ReadOnly<MatchesStore>),
}

pub struct MatchesGame {
    id: i32,
    name: String,
    logo_url: String,

    matches: Vec<Match>,
    is_loading: bool,
    is_error: bool,

    matches_store: Box<dyn Bridge<StoreWrapper<MatchesStore>>>,
}

#[derive(Properties, PartialEq)]
pub struct MatchesGameProps {
    pub id: i32,
    pub name: String,
    pub logo_url: String,
}

impl Component for MatchesGame {
    type Message = Msg;
    type Properties = MatchesGameProps;

    fn create(ctx: &Context<Self>) -> Self {
        let MatchesGameProps { id, name, logo_url } = ctx.props().clone();

        Self {
            id: id.clone(),
            name: name.clone(),
            logo_url: logo_url.clone(),

            matches: Vec::new(),
            is_loading: false,
            is_error: false,
            matches_store: MatchesStore::bridge(ctx.link().callback(Msg::MatchesStore)),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MatchesStore(state) => {
                let state = state.borrow();
                let game_id = self.id.clone();
                self.matches = state
                    .matches
                    .clone()
                    .into_iter()
                    .filter(|match_item| match_item.game_id == game_id)
                    .collect();
                self.is_loading = state.is_loading;
                self.is_error = state.is_error;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <li>
                <div class="flex gap-2 px-3 py-1 text-white font-bold bg-grey rounded-t-md">
                    if self.logo_url != "" {
                        <div class="w-6 h-6 my-auto">
                            <img src={self.logo_url.clone()} class="w-full" alt={self.name.clone()} />
                        </div>
                    }
                    {self.name.clone()}
                </div>
                <ul class="p-2 bg-light-grey rounded-b-md flex flex-col gap-2">
                    if self.is_loading {
                        <LoadingAnimation color="dark-blue" />
                    } else if self.is_error {
                        <h1>{"error"}</h1>
                    } else {{
                        self.matches.clone().into_iter().map(|match_item| {
                            let match_id = match_item.id.clone();
                            let m = match_item.clone();
                            html! {
                                <li key={ match_id }>
                                    <MatchItem
                                        id={m.id}
                                        game_id={m.game_id}
                                        team_one_name={m.team_one.unwrap().name}
                                        team_two_name={m.team_two.unwrap().name}
                                        team_one_ratio={m.team_one_ratio}
                                        team_two_ratio={m.team_two_ratio}
                                        state={m.state}
                                    />
                                </li>
                            }
                        }).collect::<Html>()
                    }}
                </ul>
            </li>
        }
    }
}
