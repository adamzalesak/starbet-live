use super::date_time::DateTime;
use crate::components::{auth::login_form::LoginForm, user::user_summary::UserSummary};
use crate::store::UserStore;
use crate::types::UserInfo;
use crate::Route;
use log::info;
use yew::prelude::*;
use yew_agent::{
    utils::store::{Bridgeable, ReadOnly, StoreWrapper},
    Bridge,
};
use yew_router::{history::Location, prelude::Link, scope_ext::RouterScopeExt};

pub enum Msg {
    SetActive(Pages),
    UserStore(ReadOnly<UserStore>),
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
    user: UserInfo,
    user_store: Box<dyn Bridge<StoreWrapper<UserStore>>>,
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
        Self {
            current_page: curr,
            user: UserInfo::new(),
            user_store: UserStore::bridge(ctx.link().callback(Msg::UserStore)),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UserStore(state) => {
                let state = state.borrow();
                self.user = state.user.clone();
            }
            Msg::SetActive(page) => self.current_page = page,
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <header class="bg-dark-blue flex flex-row justify-between text-white">
                <div onclick={ctx.link().callback(|_| Msg::SetActive(Pages::Live))} class="block w-52 header-logo transition-all my-auto">
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
                                            <Link<Route> to={Route::Live} classes={"inline-block bg-blue font-bold py-1 px-5 lg:px-9 rounded-t-lg mx-2 transition-all hover:bg-white hover:text-black"}>
                                                { "Live" }
                                            </Link<Route>>
                                        </span>
                                        <span onclick={ctx.link().callback(|_| Msg::SetActive(Pages::Upcoming))} class={if self.current_page == Pages::Upcoming {"current_active_page"} else {""}}>
                                            <Link<Route> to={Route::Upcoming} classes="inline-block bg-blue font-bold py-1 px-5 lg:px-9 rounded-t-lg mx-2 transition-all hover:bg-white hover:text-black">
                                                { "Upcoming" }
                                            </Link<Route>>
                                        </span>
                                        <span onclick={ctx.link().callback(|_| Msg::SetActive(Pages::Results))} class={if self.current_page == Pages::Results {"current_active_page"} else {""}}>
                                            <Link<Route> to={Route::Results} classes="inline-block bg-blue font-bold py-1 px-5 lg:px-9 rounded-t-lg mx-2 transition-all hover:bg-white hover:text-black">
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
                <div class="my-auto text-sm p-2">
                    {
                        if self.user.is_authenticated() {
                            html! { <UserSummary
                                first_name={self.user.first_name.clone()}
                                last_name={self.user.last_name.clone()}
                                current_balance={self.user.current_balance.clone()}
                            />
                        }
                        } else {
                            html! { <LoginForm /> }
                        }
                    }
                </div>
            </header>
        }
    }
}
