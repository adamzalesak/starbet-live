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
            <div class="font-bold mb-2">{"Games"}</div>
            <ul class="flex flex-col gap-1.5">
                <li class="text-black font-bold rounded-md bg-white p-1 text-left cursor-pointer">
                    {"League of Legends"}
                </li>
                <li class="text-black font-bold rounded-md bg-white p-1 text-left cursor-pointer">
                    {"CS:GO"}
                </li>
                <li class="text-black font-bold rounded-md bg-white p-1 text-left cursor-pointer">
                    {"Dota 2"}
                </li>
            </ul>
        </div>
        }
    }
}
