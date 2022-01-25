use super::login_form::LoginForm;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::Route;

use super::date_time::DateTime;

pub enum Msg {
    SetActive(Pages),
}

#[derive(PartialEq)]
pub enum Pages {
    None,
    Live,
    Upcoming,
    Results,
}

pub struct Header {
    current_page: Pages,
}

impl Component for Header {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        // check currently active page
        let path: String = ctx.link().location().unwrap().pathname();
        let curr: Pages = match path.as_str() {
            "/live" | "/live/" | "/" | "" => Pages::Live,
            "/results" | "/results/" => Pages::Results,
            "/upcoming" | "/upcoming/" => Pages::Upcoming,
            _ => Pages::None,
        };
        Self { current_page: curr }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetActive(page) => self.current_page = page,
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <header class="bg-dark-blue flex flex-row justify-between text-white">
                <div onclick={ctx.link().callback(|_| Msg::SetActive(Pages::Live))} class="block w-60 transition-all my-auto">
                    <Link<Route> to={Route::Home}>
                        <img src="/starbet-live-yellow.svg" alt="starbet live logo" class="p-3"/>
                    </Link<Route>>
                </div>
                <div class="flex flex-col justify-between w-full">
                    <div class="text-center my-2 text-sm">
                        <DateTime />
                    </div>
                    
                    {
                        // render navigation only when necessary 
                        if self.current_page != Pages::None {
                            html! {
                                <nav>
                                    <ul>
                                        <span onclick={ctx.link().callback(|_| Msg::SetActive(Pages::Live))} class={if self.current_page == Pages::Live {"current_active_page"} else {""}}>
                                            <Link<Route> to={Route::Live} classes={"inline-block bg-blue font-bold py-1 px-5 md:px-10 rounded-t-lg mx-2 transition-all hover:bg-white hover:text-black"}>
                                                { "Live" }
                                            </Link<Route>>
                                        </span>
                                        <span onclick={ctx.link().callback(|_| Msg::SetActive(Pages::Upcoming))} class={if self.current_page == Pages::Upcoming {"current_active_page"} else {""}}>
                                            <Link<Route> to={Route::Upcoming} classes="inline-block bg-blue font-bold py-1 px-5 md:px-10 rounded-t-lg mx-2 transition-all hover:bg-white hover:text-black">
                                                { "Upcoming" }
                                            </Link<Route>>
                                        </span>
                                        <span onclick={ctx.link().callback(|_| Msg::SetActive(Pages::Results))} class={if self.current_page == Pages::Results {"current_active_page"} else {""}}>
                                            <Link<Route> to={Route::Results} classes="inline-block bg-blue font-bold py-1 px-5 md:px-10 rounded-t-lg mx-2 transition-all hover:bg-white hover:text-black">
                                                { "Results" }
                                            </Link<Route>>
                                        </span>
                                    </ul>
                                </nav>
                            }
                        }
                        else {
                            html!{}
                        }
                    }
                </div>
                <div class="my-auto m-3">
                    <LoginForm />
                    <Link<Route> to={Route::Registration} classes="">
                        { "Join" }
                    </Link<Route>>
                </div>
            </header>
        }
    }
}
