use crate::store::{MatchesRequest, MatchesStore, TicketRequest, TicketStore};
use crate::types::grpc_types::bet::Bet;
use yew::prelude::*;
use yew::{html, Component, Html, Properties};
use yew_agent::{
    utils::store::{Bridgeable, ReadOnly, StoreWrapper},
    Bridge,
};

pub enum Msg {
    Remove,
    TicketStore(ReadOnly<TicketStore>),
    MatchesStore(ReadOnly<MatchesStore>),
}

pub struct TicketItem {
    bet: Bet,
    ticket_store: Box<dyn Bridge<StoreWrapper<TicketStore>>>,
    matches_store: Box<dyn Bridge<StoreWrapper<MatchesStore>>>,
    team_one_name: String,
    team_two_name: String,
    bet_team_name: String,
    bet_ratio: String,
}

#[derive(Properties, PartialEq)]
pub struct TicketItemProps {
    pub bet: Bet,
}

impl Component for TicketItem {
    type Message = Msg;
    type Properties = TicketItemProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            bet: ctx.props().bet.clone(),
            ticket_store: TicketStore::bridge(ctx.link().callback(Msg::TicketStore)),
            matches_store: MatchesStore::bridge(ctx.link().callback(Msg::MatchesStore)),
            team_one_name: String::new(),
            team_two_name: String::new(),
            bet_team_name: String::new(),
            bet_ratio: String::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MatchesStore(state) => {
                let state = state.borrow();
                let bet = self.bet.clone();
                let match_id = bet.match_id;
                let bet_team_id = bet.team_id;

                if let Some(match_item) = state
                    .matches_live
                    .clone()
                    .into_iter()
                    .find(|m| m.id == match_id)
                {
                    if let Some(team_one) = match_item.clone().team_one {
                        self.team_one_name = team_one.clone().name;
                        if bet_team_id == team_one.id {
                            self.bet_team_name = team_one.name;
                            self.bet_ratio = match_item.clone().team_one_ratio;
                        }
                    }
                    if let Some(team_two) = match_item.clone().team_two {
                        self.team_two_name = team_two.clone().name;
                        if bet_team_id == team_two.id {
                            self.bet_team_name = team_two.name;
                            self.bet_ratio = match_item.clone().team_two_ratio;
                        }
                    }
                } else {
                    self.ticket_store.send(TicketRequest::LoadTicket);
                }
            }
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
                        <span>{self.team_one_name.clone()}</span>
                        <span class="text-yellow">{" vs. "}</span>
                        <span>{self.team_two_name.clone()}</span>
                    </div>
                    //remove bet from ticket
                    <button type="button" class="w-3 self-start" onclick={ctx.link().callback(|_| Msg::Remove)}>
                        <img src="/remove.svg" alt="closing" class="w-full"/>
                    </button>
                </div>
                <div class="text-sm flex flex-row justify-between">
                    <div>
                        <span>{"Bet: "}</span>
                        <span class="font-bold">{self.bet_team_name.clone()}</span>
                    </div>
                    <span>{self.bet_ratio.clone()}</span>
                </div>
            </li>
        }
    }
}
