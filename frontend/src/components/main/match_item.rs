use yew::prelude::*;

pub mod team {
    include!(concat!(env!("OUT_DIR"), concat!("/team.rs")));
}
use team::Team;

pub enum Msg {}

pub struct MatchItem {
    id: i32,
    game_id: i32,
    team_one_name: String,
    team_two_name: String,
    team_one_ratio: String,
    team_two_ratio: String,
    state: String,
}

#[derive(Properties, PartialEq)]
pub struct MatchItemProps {
    pub id: i32,
    pub game_id: i32,
    pub team_one_name: String,
    pub team_two_name: String,
    pub team_one_ratio: String,
    pub team_two_ratio: String,
    pub state: String,
}

impl Component for MatchItem {
    type Message = Msg;
    type Properties = MatchItemProps;

    fn create(ctx: &Context<Self>) -> Self {
        let MatchItemProps {
            id,
            game_id,
            team_one_name,
            team_two_name,
            team_one_ratio,
            team_two_ratio,
            state,
        } = ctx.props().clone();

        Self {
            id: id.clone(),
            game_id: game_id.clone(),
            team_one_name: team_one_name.clone(),
            team_two_name: team_two_name.clone(),
            team_one_ratio: team_one_ratio.clone(),
            team_two_ratio: team_two_ratio.clone(),
            state: state.clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="px-2 bg-white rounded-md flex flex-col lg:flex-row lg:gap-2 pb-2 relative">
                <div class="w-1/3">
                    <div class="font-bold absolute">
                        <span>{"Fnatic"}</span>
                        {" "}
                        <span class="text-yellow">{"13 kills - 17 kills"}</span>
                        {" "}
                        <span>{"SKT T1"}</span>
                    </div>
                    <div class="mt-5 w-full">{"Time -> 25:31"}</div>
                </div>
                <div class="w-full lg:w-2/3">
                    <div class="text-center">{"Winner"}</div>
                    <div class="flex gap-5">
                        <button class="bg-light-grey rounded-md w-full p-1 px-3 flex gap-1 justify-between cursor-pointer">
                            <span class="font-bold">{"Fnatic"}</span>
                            <span>{"2.15"}</span>
                        </button>
                        <button class="bg-light-grey rounded-md w-full p-1 px-3 flex gap-1 justify-between cursor-pointer">
                            <span class="font-bold">{"SKT T1"}</span>
                            <span>{"1.63"}</span>
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}
