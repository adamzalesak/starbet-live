use super::ticket_item::TicketItem;
use crate::components::loading_animation::LoadingAnimation;
use crate::store::{
    MatchesRequest, MatchesStore, TicketRequest, TicketStore, UserRequest, UserStore,
};
use crate::types::grpc_types::{bet::Bet, game_match::Match};
use gloo::console::info;
use gloo_timers::callback::Timeout;
use std::collections::HashSet;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_agent::{
    utils::store::{Bridgeable, ReadOnly, StoreWrapper},
    Bridge,
};

pub enum Msg {
    ChangeValue(Event),
    Submit,
    RefreshRate,
    TicketStore(ReadOnly<TicketStore>),
    MatchesStore(ReadOnly<MatchesStore>),
    UserStore(ReadOnly<UserStore>),
}

pub struct Ticket {
    bets: Vec<Bet>,
    rate: f32,
    ticket_value: f32,

    live_matches: Vec<Match>,

    ticket_store: Box<dyn Bridge<StoreWrapper<TicketStore>>>,
    matches_store: Box<dyn Bridge<StoreWrapper<MatchesStore>>>,
    user_store: Box<dyn Bridge<StoreWrapper<UserStore>>>,
    ticket_is_loading: bool,
    matches_is_loading: bool,
}

// parse value from event type
fn get_value_from_event(e: Event) -> String {
    let event_target = e.target().unwrap_throw();
    let target: HtmlInputElement = event_target.dyn_into().unwrap_throw();
    target.value()
}

impl Component for Ticket {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            bets: Vec::new(),
            rate: 1.0,
            ticket_value: 1.0,

            live_matches: Vec::new(),

            ticket_store: TicketStore::bridge(ctx.link().callback(Msg::TicketStore)),
            matches_store: MatchesStore::bridge(ctx.link().callback(Msg::MatchesStore)),
            user_store: UserStore::bridge(ctx.link().callback(Msg::UserStore)),
            ticket_is_loading: false,
            matches_is_loading: false,
        }
    }

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            self.ticket_store.send(TicketRequest::LoadTicket);
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MatchesStore(state) => {
                let state = state.borrow();
                self.live_matches = state.matches_live.clone();
                self.matches_is_loading = state.is_loading;

                ctx.link().send_message(Msg::RefreshRate);
            }

            Msg::TicketStore(state) => {
                let state = state.borrow();

                // refresh rate only if bets had changed
                let x: HashSet<i32> = self
                    .bets
                    .clone()
                    .into_iter()
                    .map(|b| b.id)
                    .collect::<HashSet<i32>>();
                let y: HashSet<i32> = state
                    .bets
                    .clone()
                    .into_iter()
                    .map(|b| b.id)
                    .collect::<HashSet<i32>>();
                if x != y {
                    ctx.link().send_message(Msg::RefreshRate);
                }

                self.bets = state.bets.clone();
                self.ticket_value = state.ticket_value;
                self.rate = state.rate;
                self.ticket_is_loading = state.is_loading;
            }

            Msg::UserStore(_) => {}

            // check if value is type of f32, otherwise wet bet_value to 1.0
            Msg::ChangeValue(data) => {
                let val = get_value_from_event(data);
                let value = match val.parse::<f32>() {
                    Ok(value) => value,
                    _ => 1.0,
                };
                self.ticket_store
                    .send(TicketRequest::ChangeTicketValue(value))
            }

            Msg::RefreshRate => {
                let mut result: f32 = 1.0;

                self.bets.clone().into_iter().for_each(|b| {
                    if let Some(match_item) = self
                        .live_matches
                        .clone()
                        .into_iter()
                        .find(|m| m.id == b.match_id)
                    {
                        if b.team_id == match_item.team_one.unwrap().id {
                            if let Ok(team_ratio) = match_item.team_one_ratio.parse::<f32>() {
                                result *= team_ratio;
                            }
                        } else if b.team_id == match_item.team_two.unwrap().id {
                            if let Ok(team_ratio) = match_item.team_two_ratio.parse::<f32>() {
                                result *= team_ratio;
                            }
                        }
                    }
                });

                self.ticket_store.send(TicketRequest::UpdateRate(result));
            }

            Msg::Submit => {
                self.ticket_store.send(TicketRequest::SubmitTicket);
                self.user_store.send(UserRequest::InitializeUser);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
         <div class="bg-dark-blue text-white rounded-md p-1 h-full max-h-full">
            <div class="bg-light-grey rounded-md text-black flex flex-col h-full">
                <div class="font-bold text-center pt-1 pb-3 bg-dark-blue text-white">{"Current ticket"}</div>
                <div class="overflow-auto m-1 mb-auto bg-light-grey">
                    if self.ticket_is_loading || self.matches_is_loading {
                        <LoadingAnimation color="dark-blue" />
                    } else {
                        <ul>
                            {
                                if self.bets.is_empty() {
                                    html!{ <div>{"Your current ticket is empty!"}</div>}
                                } else {
                                    html! {}
                                }
                            }
                            {
                                for self.bets.iter().map(|x| html! { <li key={x.clone().id}><TicketItem bet={x.clone()} /></li> })
                            }
                        </ul>
                    }
                </div>

                <form class="m-1" onsubmit={ctx.link().callback(|e: FocusEvent| { e.prevent_default(); Msg::Submit })} >
                    <div class="text-sm">{ "Number of matches: " }{ self.bets.len() }</div>
                    <input type="number"
                        min="1.0"
                        step="0.5"
                        id="amount"
                        placeholder="1.0"
                        class="block w-6/12 rounded-md p-1"
                        onchange={ctx.link().callback(Msg::ChangeValue)}
                        />
                    <button type="submit" class="bg-yellow w-full rounded-t-md p-1 font-bold mt-1 transition-all">{self.ticket_value}{" €"}</button>
                    <div class="flex flex-row justify-between text-sm bg-dark-yellow rounded-b-md p-1">
                        <span>{"Total ratio: "}{self.rate}</span>
                        <span>{"ev.win: "}{self.ticket_value * self.rate}{"€"}</span>
                    </div>
                </form>


            </div>
         </div>
        }
    }
}
