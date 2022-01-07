use yew::prelude::*;

pub enum Msg {}

pub struct LatestTickets {}

impl Component for LatestTickets {
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
         <div class="bg-dark-blue text-white rounded-md  p-2">{"latest tickets"}</div>
        }
    }
}
