use crate::types::grpc_types::team::Team;
use chrono::{NaiveDateTime};
use yew::prelude::*;

pub enum Msg {}

pub struct MatchItemUpcoming {
    id: i32,
    game_id: i32,
    team_one_name: String,
    team_two_name: String,
    team_one_ratio: String,
    team_two_ratio: String,
    state: String,
    supposed_start_at: String,
}

#[derive(Properties, PartialEq)]
pub struct MatchItemUpcomingProps {
    pub id: i32,
    pub game_id: i32,
    pub team_one_name: String,
    pub team_two_name: String,
    pub team_one_ratio: String,
    pub team_two_ratio: String,
    pub state: String,
    pub supposed_start_at: String,
}

impl Component for MatchItemUpcoming {
    type Message = Msg;
    type Properties = MatchItemUpcomingProps;

    fn create(ctx: &Context<Self>) -> Self {
        let MatchItemUpcomingProps {
            id,
            game_id,
            team_one_name,
            team_two_name,
            team_one_ratio,
            team_two_ratio,
            state,
            supposed_start_at,
        } = ctx.props().clone();

        Self {
            id: id.clone(),
            game_id: game_id.clone(),
            team_one_name: team_one_name.clone(),
            team_two_name: team_two_name.clone(),
            team_one_ratio: team_one_ratio.clone(),
            team_two_ratio: team_two_ratio.clone(),
            state: state.clone(),
            supposed_start_at: supposed_start_at.clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let starts_at =
            NaiveDateTime::parse_from_str(&self.supposed_start_at, "%Y-%m-%d %H:%M:%S%.9f UTC")
                .unwrap();
        let starts_at_date = starts_at.format("%d/%m/%Y").to_string();
        let starts_at_time = starts_at.format("%H:%M").to_string();

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
                    <div class="lg:mt-6 w-full">
                        {"Starts on "}
                        <span class="font-bold">{starts_at_date}</span>
                        {" at "}
                        <span class="font-bold">{starts_at_time}</span>
                    </div>
                </div>
                <div class="w-full lg:w-2/3">
                    <div class="text-center">{"Opening betting odds"}</div>
                    <div class="flex gap-5">
                        <div class="bg-light-grey rounded-md w-full p-1 px-3 flex gap-1 justify-between">
                            <span class="font-bold">{self.team_one_name.clone()}</span>
                            <span>{self.team_one_ratio.clone()}</span>
                        </div>
                        <div class="bg-light-grey rounded-md w-full p-1 px-3 flex gap-1 justify-between">
                            <span class="font-bold">{self.team_two_name.clone()}</span>
                            <span>{self.team_two_ratio.clone()}</span>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}
