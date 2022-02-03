use super::{
    match_item_ended::MatchItemEnded, match_item_live::MatchItemLive,
    match_item_upcoming::MatchItemUpcoming,
};
use crate::components::loading_animation::LoadingAnimation;
use crate::store::{MatchesRequest, MatchesStore};
use crate::types::grpc_types::game_match::{
    match_service_client, GameEventType, ListMatchesReply, Match,
};
use crate::types::grpc_types::team::Team;
use anyhow;
use yew::prelude::*;
use yew_agent::{
    utils::store::{Bridgeable, ReadOnly, StoreWrapper},
    Bridge,
};

pub enum Msg {
    MatchesStore(ReadOnly<MatchesStore>),
}

pub struct GameMatches {
    event_type: GameEventType,

    id: i32,
    name: String,
    logo_url: String,

    matches: Vec<Match>,
    is_loading: bool,
    is_error: bool,

    matches_store: Box<dyn Bridge<StoreWrapper<MatchesStore>>>,
}

#[derive(Properties, PartialEq)]
pub struct GameMatchesProps {
    pub event_type: GameEventType,
    pub id: i32,
    pub name: String,
    pub logo_url: String,
}

impl Component for GameMatches {
    type Message = Msg;
    type Properties = GameMatchesProps;

    fn create(ctx: &Context<Self>) -> Self {
        let GameMatchesProps {
            event_type,
            id,
            name,
            logo_url,
        } = ctx.props().clone();

        Self {
            event_type: event_type.clone(),
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
                let matches = if self.event_type == GameEventType::Upcoming {
                    state.matches_upcoming.clone()
                } else if self.event_type == GameEventType::Live {
                    state.matches_live.clone()
                } else {
                    state.matches_ended.clone()
                };
                self.matches = matches
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
        if self.matches.is_empty() {
            html! {}
        } else {
            html! {
                <div class="pb-2">
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
                            {"error"}
                        } else {{
                            self.matches.clone().into_iter().map(|match_item| {
                                let match_id = match_item.id.clone();
                                let m = match_item.clone();

                                let mut winner_name: String = String::from("");
                                if let Some(winner_id) = m.winner_id {
                                    if m.team_one.clone().unwrap().id == winner_id {
                                        winner_name = m.team_one.clone().unwrap().name;
                                    } else if m.team_two.clone().unwrap().id == winner_id {
                                        winner_name = m.team_two.clone().unwrap().name;
                                    }
                                }

                                html! {
                                    <li key={ match_id }>
                                        if self.event_type == GameEventType::Upcoming {
                                            <MatchItemUpcoming
                                                id={m.id}
                                                game_id={m.game_id}
                                                team_one_name={m.team_one.unwrap().name}
                                                team_two_name={m.team_two.unwrap().name}
                                                team_one_ratio={m.team_one_ratio}
                                                team_two_ratio={m.team_two_ratio}
                                                state={m.state}
                                                supposed_start_at={m.supposed_start_at}
                                            />
                                        } else if self.event_type == GameEventType::Live {
                                            <MatchItemLive
                                                id={m.id}
                                                game_id={m.game_id}
                                                team_one_id={m.team_one.clone().unwrap().id}
                                                team_two_id={m.team_two.clone().unwrap().id}
                                                team_one_name={m.team_one.clone().unwrap().name}
                                                team_two_name={m.team_two.clone().unwrap().name}
                                                team_one_ratio={m.team_one_ratio}
                                                team_two_ratio={m.team_two_ratio}
                                                state={m.state}
                                            />
                                        } else if self.event_type == GameEventType::Ended {
                                            <MatchItemEnded
                                                id={m.id}
                                                game_id={m.game_id}
                                                team_one_name={m.team_one.unwrap().name}
                                                team_two_name={m.team_two.unwrap().name}
                                                team_one_ratio={m.team_one_ratio}
                                                team_two_ratio={m.team_two_ratio}
                                                state={m.state}
                                                supposed_start_at={m.supposed_start_at}
                                                winner_name={winner_name}
                                            />
                                        }

                                    </li>
                                }
                            }).collect::<Html>()
                        }}
                    </ul>
                </div>
            }
        }
    }
}
