use yew::prelude::*;
use yew::{html, Component, Html, Properties};

pub enum Msg {
}

pub struct TicketItem {}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub id: i32,
    pub team1: String,
    pub team2: String,
    pub bet: String,
    pub rate: f32,
}

impl Component for TicketItem {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <li class="rounded-md border border-dark-blue p-1 mb-1">
                <div class="font-bold flex flex-row justify-between">
                    <div>
                        <span>{ctx.props().team1.clone()}</span>
                        <span class="text-yellow">{" vs. "}</span>
                        <span>{ctx.props().team2.clone()}</span>
                    </div>
                    //remove bet from ticket
                    <button type="button" class="w-3 self-start">
                        <img src="/remove.svg" alt="closing" class="w-full"/>
                    </button>
                </div>
                <div class="text-sm flex flex-row justify-between">
                    <div>
                        <span>{"Bet: "}</span>
                        <span class="font-bold">{ctx.props().bet.clone()}</span>
                    </div>
                    <span>{ctx.props().rate}</span>
                </div>
            </li>
        }
    }
}
