use yew::prelude::*;

pub enum Msg {}

pub struct Games {}

impl Component for Games {
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
         <div class="bg-dark-blue text-white flex-auto rounded-md p-2 text-center">
            <span class="font-bold">{"Games"}</span>
            <div>
                <div></div>
                <div></div>
            </div>
        </div>
        }
    }
}
