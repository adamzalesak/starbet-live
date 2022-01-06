use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

use super::date_time::DateTime;

pub enum Msg {
    setActive,
}

enum Tabs {
    Live,
    Upcoming,
    Results,
}

pub struct Header {
    is_logged: bool,
    current_tab: Tabs,
}

impl Component for Header {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            is_logged: false,
            current_tab: Tabs::Live,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let history = ctx.link().location().unwrap().pathname();
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <header class="bg-dark-blue flex flex-row justify-between text-white">
                <Link<Route> to={Route::Home} classes="block w-4/12 md:w-3/12 transition-all p-3 my-auto">
                    <img src="assets/icons/starbet-live-yellow.svg" alt="starbet live logo" />
                </Link<Route>>
                <div class="flex flex-col justify-between w-full">
                    <div class="text-center my-2 text-sm">
                        <DateTime />
                    </div>
                    <nav>
                        <ul>
                            <Link<Route> to={Route::Live} classes={"inline-block bg-blue font-bold text-white py-1 px-5 md:px-10 rounded-t-lg mx-2 transition-all hover:bg-white hover:text-black"}>
                                { "Live" }
                            </Link<Route>>
                            <Link<Route> to={Route::Upcoming} classes="inline-block bg-blue font-bold text-white py-1 px-5 md:px-10 rounded-t-lg mx-2 transition-all hover:bg-white hover:text-black">
                                { "Upcoming" }
                            </Link<Route>>
                            <Link<Route> to={Route::Results} classes="inline-block bg-blue font-bold text-white py-1 px-5 md:px-10 rounded-t-lg mx-2 transition-all hover:bg-white hover:text-black">
                                { "Results" }
                            </Link<Route>>
                        </ul>
                    </nav>
                </div>
                <div class="my-auto p-3">
                    <span>{"Login form"}</span>
                </div>
            </header>
        }
    }
}
