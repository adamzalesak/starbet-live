use crate::types::grpc_types::team::Team;
use chrono::NaiveDateTime;
use yew::prelude::*;

pub enum Msg {}

pub struct MatchItemEnded {
    id: i32,
    game_id: i32,
    team_one_name: String,
    team_two_name: String,
    team_one_ratio: String,
    team_two_ratio: String,
    state: String,
    supposed_start_at: String,
    winner_name: String,
}

#[derive(Properties, PartialEq)]
pub struct MatchItemEndedProps {
    pub id: i32,
    pub game_id: i32,
    pub team_one_name: String,
    pub team_two_name: String,
    pub team_one_ratio: String,
    pub team_two_ratio: String,
    pub state: String,
    pub supposed_start_at: String,
    pub winner_name: String,
}

impl Component for MatchItemEnded {
    type Message = Msg;
    type Properties = MatchItemEndedProps;

    fn create(ctx: &Context<Self>) -> Self {
        let MatchItemEndedProps {
            id,
            game_id,
            team_one_name,
            team_two_name,
            team_one_ratio,
            team_two_ratio,
            state,
            supposed_start_at,
            winner_name,
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
            winner_name: winner_name.clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let starts_at_date_time =
            NaiveDateTime::parse_from_str(&self.supposed_start_at, "%Y-%m-%d %H:%M:%S%.9f UTC")
                .unwrap()
                .format("%d/%m/%Y %H:%M")
                .to_string();

        html! {
            <div class="px-2 bg-white rounded-md flex flex-col lg:flex-row justify-between lg:gap-2 pb-2">
                <div class="lg:w-1/3">
                    <div class="font-bold">
                        <span>{self.team_one_name.clone()}</span>
                        {" "}
                        <span class="text-yellow">{self.state.clone()}</span>
                        {" "}
                        <span>{self.team_two_name.clone()}</span>
                    </div>
                    <div>
                        {"Winner: "}
                        <span class="font-bold">{self.winner_name.clone()}</span>
                    </div>
                </div>
                <div class="font-bold">{starts_at_date_time}</div>
            </div>
        }
    }
}
