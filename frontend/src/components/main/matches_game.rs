use yew::prelude::*;

use super::match_item::MatchItem;

pub enum Msg {}

pub struct MatchesGame {}

impl Component for MatchesGame {
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
            <li>
                <div class="px-3 py-1 text-white font-bold bg-grey rounded-t-md">
                    {"League of Legends"}
                </div>
                <div class="p-2 bg-light-grey rounded-b-md flex flex-col gap-2">
                    <MatchItem />
                    <MatchItem />
                    <MatchItem />
                </div>
            </li>
        }
    }
}
