use yew::prelude::*;

use crate::components::main::matches::Matches;

pub enum Msg {}

pub struct LivePage {}

impl Component for LivePage {
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
          <Matches />
        }
    }
}
