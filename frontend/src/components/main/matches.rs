use yew::prelude::*;

use super::matches_game::MatchesGame;

pub enum Msg {}
pub struct Matches {}

impl Component for Matches {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <ul class="flex flex-col gap-2 overflow-auto">
                <MatchesGame />
                <MatchesGame />
                <MatchesGame />
                <MatchesGame />
            </ul>
        }
    }
}
