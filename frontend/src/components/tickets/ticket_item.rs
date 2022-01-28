use crate::agents::tickets::{TicketRequest, TicketStore};
use crate::types::tickets::BetInfo;
use yew::prelude::*;
use yew::{html, Component, Html, Properties};
use yew_agent::{
    utils::store::{Bridgeable, ReadOnly, StoreWrapper},
    Bridge,
};

pub enum Msg {
    Remove,
    TicketStore(ReadOnly<TicketStore>),
}

pub struct TicketItem {
    bet: BetInfo,
    ticket_store: Box<dyn Bridge<StoreWrapper<TicketStore>>>,
}

#[derive(Properties, PartialEq)]
pub struct TicketItemProps {
    pub bet: BetInfo,
}

impl Component for TicketItem {
    type Message = Msg;
    type Properties = TicketItemProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            bet: ctx.props().bet.clone(),
            ticket_store: TicketStore::bridge(ctx.link().callback(Msg::TicketStore)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Remove => self
                .ticket_store
                .send(TicketRequest::DeleteBet(self.bet.id)),
            _ => {}
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <li class="rounded-md border border-dark-blue p-1 mb-1">
                <div class="font-bold flex flex-row justify-between">
                    <div>
                        <span>{&self.bet.team1}</span>
                        <span class="text-yellow">{" vs. "}</span>
                        <span>{&self.bet.team2}</span>
                    </div>
                    //remove bet from ticket
                    <button type="button" class="w-3 self-start" onclick={ctx.link().callback(|_| Msg::Remove)}>
                        <img src="/remove.svg" alt="closing" class="w-full"/>
                    </button>
                </div>
                <div class="text-sm flex flex-row justify-between">
                    <div>
                        <span>{"Bet: "}</span>
                        <span class="font-bold">{&self.bet.bet_team}</span>
                    </div>
                    <span>{&self.bet.bet_ratio}</span>
                </div>
            </li>
        }
    }
}
