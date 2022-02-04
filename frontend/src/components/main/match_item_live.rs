use crate::store::{TicketRequest, TicketStore};
use crate::types::grpc_types::team::Team;
use yew::prelude::*;
use yew_agent::{
    utils::store::{Bridgeable, ReadOnly, StoreWrapper},
    Bridge,
};

pub enum Msg {
    CreateBetTeamOne,
    CreateBetTeamTwo,
    TicketStore(ReadOnly<TicketStore>),
}

pub struct MatchItemLive {
    id: i32,
    game_id: i32,
    team_one_id: i32,
    team_two_id: i32,
    team_one_name: String,
    team_two_name: String,
    team_one_ratio: String,
    team_two_ratio: String,
    state: String,

    ticket_store: Box<dyn Bridge<StoreWrapper<TicketStore>>>,
}

#[derive(Properties, PartialEq)]
pub struct MatchItemLiveProps {
    pub id: i32,
    pub game_id: i32,
    pub team_one_id: i32,
    pub team_two_id: i32,
    pub team_one_name: String,
    pub team_two_name: String,
    pub team_one_ratio: String,
    pub team_two_ratio: String,
    pub state: String,
}

impl Component for MatchItemLive {
    type Message = Msg;
    type Properties = MatchItemLiveProps;

    fn create(ctx: &Context<Self>) -> Self {
        let MatchItemLiveProps {
            id,
            game_id,
            team_one_id,
            team_two_id,
            team_one_name,
            team_two_name,
            team_one_ratio,
            team_two_ratio,
            state,
        } = ctx.props().clone();

        Self {
            id: id.clone(),
            game_id: game_id.clone(),
            team_one_id: team_one_id.clone(),
            team_two_id: team_two_id.clone(),
            team_one_name: team_one_name.clone(),
            team_two_name: team_two_name.clone(),
            team_one_ratio: team_one_ratio.clone(),
            team_two_ratio: team_two_ratio.clone(),
            state: state.clone(),

            ticket_store: TicketStore::bridge(ctx.link().callback(Msg::TicketStore)),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::TicketStore(state) => {
                let state = state.borrow();
            }
            Msg::CreateBetTeamOne => {
                self.ticket_store.send(TicketRequest::CreateBet(
                    self.id.clone(),
                    self.team_one_id.clone(),
                ));
            }
            Msg::CreateBetTeamTwo => {
                self.ticket_store.send(TicketRequest::CreateBet(
                    self.id.clone(),
                    self.team_two_id.clone(),
                ));
            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="px-2 bg-white rounded-md flex flex-col lg:flex-row lg:gap-2 pb-2 lg:relative">
                <div class="lg:w-1/3">
                    <div class="font-bold lg:absolute">
                        <span>{self.team_one_name.clone()}</span>
                        {" "}
                        <span class="text-yellow">{self.state.clone()}</span>
                        {" "}
                        <span>{self.team_two_name.clone()}</span>
                    </div>
                    // <div class="mt-5 w-full">{"Time -> 25:31"}</div>
                </div>
                <div class="w-full lg:w-2/3">
                    <div class="text-center">{"Winner"}</div>
                    <div class="flex gap-5">
                        <button onclick={ctx.link().callback(|_| Msg::CreateBetTeamOne)} class="bg-light-grey rounded-md w-full p-1 px-3 flex gap-1 justify-between cursor-pointer">
                            <span class="font-bold">{self.team_one_name.clone()}</span>
                            <span>{self.team_one_ratio.clone()}</span>
                        </button>
                        <button onclick={ctx.link().callback(|_| Msg::CreateBetTeamTwo)} class="bg-light-grey rounded-md w-full p-1 px-3 flex gap-1 justify-between cursor-pointer">
                            <span class="font-bold">{self.team_two_name.clone()}</span>
                            <span>{self.team_two_ratio.clone()}</span>
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}
