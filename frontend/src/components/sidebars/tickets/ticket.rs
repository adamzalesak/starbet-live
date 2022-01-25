use super::ticket_item::TicketItem;
use crate::{
    agents::tickets::{TicketRequest, TicketStore},
    types::tickets::BetInfo,
};
use log::info;
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
    TicketStore(ReadOnly<TicketStore>),
}

pub struct Ticket {
    bets: Vec<BetInfo>,
    rate: f32,
    ticket_value: f32,
    ticket_store: Box<dyn Bridge<StoreWrapper<TicketStore>>>,
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
            ticket_store: TicketStore::bridge(ctx.link().callback(Msg::TicketStore)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            // check if value is type of f32, otherwise wet bet_value to 1.0
            Msg::ChangeValue(data) => {
                let value = match get_value_from_event(data) {
                    val => match val.parse::<f32>() {
                        Ok(value) => value,
                        _ => 1.0,
                    },
                };
                self.ticket_store
                    .send(TicketRequest::ChangeTicketValue(value))
            }
            Msg::Submit => {
                self.ticket_store.send(TicketRequest::SubmitTicket);
            },
            Msg::TicketStore(state) => {
                info!("Received update");

                let state = state.borrow();

                if state.bets.len() != self.bets.len() {
                    self.bets = state.bets.values().cloned().collect();
                }
                self.ticket_value = state.ticket_value;
                self.rate = state.rate;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
         <div class="bg-dark-blue text-white rounded-md p-1 h-4/6 max-h-full">
            <div class="bg-light-grey rounded-md p-1 text-black flex flex-col h-full">
                <div class="font-bold text-center mb-3 mt-1 bg-dark-blue">{"Current ticket"}</div>
                <ul class="overflow-auto mb-auto">
                    {
                        if self.bets.len() == 0 {
                            html!{ <div>{"Your current ticket is empty!"}</div>}
                        } else {
                            html! {}
                        }
                    }
                    {
                        for self.bets.iter().map(|x| html! { <TicketItem bet={x.clone()} /> })
                    }
                </ul>

                <form class="mt-1" onsubmit={ctx.link().callback(|e: FocusEvent| { e.prevent_default(); Msg::Submit })} >
                    <div class="text-sm">{ "Number of matches: " }{ self.bets.len() }</div>
                    <input type="number"
                        min="1.0"
                        step="0.5"
                        id="amount"
                        placeholder="1.0"
                        class="block w-6/12 rounded-md p-1"
                        onchange={ctx.link().callback(|data: Event| Msg::ChangeValue(data))}
                        />
                    <button type="submit" class="bg-yellow w-full rounded-t-md p-1 font-bold mt-1">{self.ticket_value}{" €"}</button>
                    <div class="flex flex-row justify-between text-sm bg-dark-yellow rounded-b-md p-1">
                        <span>{"Rate: "}{self.rate}</span>
                        <span>{"ev.win: "}{self.ticket_value * self.rate}{"€"}</span>
                    </div>
                </form>


            </div>
         </div>
        }
    }
}
