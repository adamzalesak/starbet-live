use yew::prelude::*;
use yew_router::prelude::Link;

use crate::Route;

pub enum Msg {}

pub struct Footer {}

impl Component for Footer {
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
            <div class="bg-dark-blue text-white text-center text-sm rounded-md p-2">
                <span class="block my-2">{"© Starbet Live 2021"}</span>
                <div class="flex flex-col my-2"> 
                    <Link<Route> to={Route::Home} classes={"underline my-1"}>
                        {"About"}
                    </Link<Route>>
                    <Link<Route> to={Route::Home} classes={"underline my-1"}>
                        {"Privacy Policy"}
                    </Link<Route>>
                    <Link<Route> to={Route::Home} classes={"underline my-1"}>
                        {"Contact"}
                    </Link<Route>>
                </div>
            </div>
        }
    }
}
