use yew::html::*;
use yew::prelude::*;

pub enum Msg {
    ChangeBet(InputEvent),
}

pub struct Ticket {
    rate: f32,
    bet_value: f32,
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
                self.bet_value = match data.data() {
                    Some(val) => match val.parse::<f32>() {
                        Ok(value) => value,
                        _ => 1.0,
                    },
                    _ => 1.0,
                };
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
         <div class="bg-dark-blue text-white rounded-md p-1">
            <div class="font-bold text-center mb-3 mt-1">{"Current ticket"}</div>
            <div class="bg-light-grey rounded-md p-1 text-black">
                <ul>
                    <li class="rounded-md border border-dark-blue p-1 mb-1">
                        <div class="font-bold">
                            <span>{"Team 1"}</span>
                            <span class="text-yellow">{" vs. "}</span>
                            <span>{"Team 2"}</span>
                        </div>
                        <div class="text-sm flex flex-row justify-between">
                            <div>
                                <span>{"Bet: "}</span>
                                <span class="font-bold">{"Team 1"}</span>
                            </div>
                            <span>{"Ratio"}</span>
                        </div>
                    </li>
                    <li class="rounded-md border border-dark-blue p-1 mb-1">
                        <div class="font-bold">
                            <span>{"Team 1"}</span>
                            <span class="text-yellow">{" vs. "}</span>
                            <span>{"Team 2"}</span>
                        </div>
                        <div class="text-sm flex flex-row justify-between">
                            <div>
                                <span>{"Bet: "}</span>
                                <span class="font-bold">{"Team 1"}</span>
                            </div>
                            <span>{"Ratio"}</span>
                        </div>
                    </li>
                </ul>
                <div class="text-sm">{"Number of matches: "}{"2"}</div>
                <form class="mt-1">
                    <input type="number" min="1" name="amount"
                        value={self.bet_value.to_string()}
                        class="block w-6/12 rounded-md p-1"
                        step="0.5"
                        oninput={ctx.link().callback(|data: InputEvent| Msg::ChangeBet(data))}
                        />
                    <button type="submit" class="bg-yellow w-full rounded-t-md p-1 font-bold mt-1">{self.bet_value}{" €"}</button>
                    <div class="flex flex-row justify-between text-sm bg-dark-yellow rounded-b-md p-1">
                        <span>{"Rate: "}{self.rate}</span>
                        <span>{"ev.win: "}{self.bet_value}{"€"}</span>
                    </div>
                </form>


            </div>
         </div>
        }
    }
}
