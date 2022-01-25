use super::ticket_item::TicketItem;
use wasm_bindgen::{JsCast, UnwrapThrowExt};
use web_sys::HtmlInputElement;
use yew::prelude::*;

pub enum Msg {
    ChangeBet(Event),
    Submit,
}

pub struct Ticket {
    rate: f32,
    bet_value: f32,
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

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            rate: 1.0,
            bet_value: 1.0,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            // check if value is type of f32, otherwise wet bet_value to 1.0
            Msg::ChangeBet(data) => {
                self.bet_value = match get_value_from_event(data) {
                    val => match val.parse::<f32>() {
                        Ok(value) => value,
                        _ => 1.0,
                    },
                    _ => 1.0,
                };
            }
            Msg::Submit => todo!(),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
         <div class="bg-dark-blue text-white rounded-md p-1 h-4/6 max-h-full">
            <div class="bg-light-grey rounded-md p-1 text-black flex flex-col h-full">
                <div class="font-bold text-center mb-3 mt-1">{"Current ticket"}</div>
                <ul class="overflow-auto mb-auto">
                    <TicketItem id={1} team1="Team 1" team2="Team 2" bet="Team 1" rate={3.2} />
                    <TicketItem id={2} team1="Team 1" team2="Team 2" bet="Team 1" rate={3.2} />
                </ul>

                <form class="mt-1" onsubmit={ctx.link().callback(|e: FocusEvent| { e.prevent_default(); Msg::Submit })} >
                    <div class="text-sm">{"Number of matches: "}{"2"}</div>
                    <input type="number"
                        step="0.5"
                        id="amount"
                        class="block w-6/12 rounded-md p-1"
                        value={self.bet_value.to_string()}
                        onchange={ctx.link().callback(|data: Event| Msg::ChangeBet(data))}
                        />
                    <button type="submit" class="bg-yellow w-full rounded-t-md p-1 font-bold mt-1">{self.bet_value}{" €"}</button>
                    <div class="flex flex-row justify-between text-sm bg-dark-yellow rounded-b-md p-1">
                        <span>{"Rate: "}{self.rate}</span>
                        <span>{"ev.win: "}{self.bet_value * self.rate}{"€"}</span>
                    </div>
                </form>


            </div>
         </div>
        }
    }
}
