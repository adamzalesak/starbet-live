use super::ticket_item::TicketItem;
use crate::store::{TicketRequest, TicketStore};
use crate::types::grpc_types::bet::Bet;
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
    bets: Vec<Bet>,
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

    fn rendered(&mut self, _ctx: &Context<Self>, first_render: bool) {
        if first_render {
            self.ticket_store.send(TicketRequest::LoadTicket);
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::TicketStore(state) => {
                let state = state.borrow();

                self.bets = state.bets.clone();

                self.ticket_value = state.ticket_value;
                self.rate = state.rate;
            }
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
            Msg::Submit => {
                self.ticket_store.send(TicketRequest::SubmitTicket);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
         <div class="bg-dark-blue text-white rounded-md p-1 h-full max-h-full">
            <div class="bg-light-grey rounded-md text-black flex flex-col h-full">
                <div class="font-bold text-center pt-1 pb-3 bg-dark-blue text-white">{"Current ticket"}</div>
                <ul class="overflow-auto m-1 mb-auto bg-light-grey">
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
                        <span>{"Rate: "}{self.rate}</span>
                        <span>{"ev.win: "}{self.ticket_value * self.rate}{"€"}</span>
                    </div>
                </form>


            </div>
         </div>
        }
    }
}
