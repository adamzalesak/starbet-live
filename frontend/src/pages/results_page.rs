use crate::components::main::matches_ended::MatchesEnded;
use yew::prelude::*;

pub enum Msg {}

pub struct ResultsPage {}

impl Component for ResultsPage {
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
         <MatchesEnded />
        }
    }
}
