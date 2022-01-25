use yew::prelude::*;

pub enum Msg {}

pub struct ContactPage {}

impl Component for ContactPage {
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
         <div class="w-full lg:w-8/12 p-2 sm:w-10/12 overflow-auto mx-auto transition-all">{"contact page"}</div>
        }
    }
}
