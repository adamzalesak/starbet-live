use super::date_time::DateTime;
use crate::components::{auth::login_form::LoginForm, user::user_summary::UserSummary};
use crate::store::UserStore;
use crate::types::MainRoute;
use crate::types::UserInfo;
use yew::prelude::*;
use yew_agent::{
    utils::store::{Bridgeable, ReadOnly, StoreWrapper},
    Bridge,
};
use yew_router::{prelude::Link, scope_ext::RouterScopeExt};

pub enum Msg {
    SetCurrentTab,
    UserStore(ReadOnly<UserStore>),
}

pub struct Header {
    current_tab: Option<MainRoute>,
    user: UserInfo,
    user_store: Box<dyn Bridge<StoreWrapper<UserStore>>>,
}

impl Component for Header {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            current_tab: ctx.link().route::<MainRoute>(),
            user: UserInfo::new(),
            user_store: UserStore::bridge(ctx.link().callback(Msg::UserStore)),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UserStore(state) => {
                let state = state.borrow();
                self.user = state.user.clone();
            }
            Msg::SetCurrentTab => self.current_tab = ctx.link().route::<MainRoute>(),
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <header class="bg-dark-blue flex flex-row justify-between text-white">
                <div onclick={ctx.link().callback(|_| Msg::SetCurrentTab)} class="block w-52 header-logo transition-all my-auto">
                    <Link<MainRoute> to={MainRoute::Home}>
                        <img src="/starbet-live-yellow.svg" alt="starbet live logo" class="p-3"/>
                    </Link<MainRoute>>
                </div>
                <div class="flex flex-col justify-between w-full">
                    <div class="text-center my-2 text-sm">
                        <DateTime />
                    </div>

                    {
                        match self.current_tab {
                            Some(MainRoute::Live) | Some(MainRoute::Results) | Some(MainRoute::Upcoming) | Some(MainRoute::Home) => { 
                                html! {
                                    <nav>
                                        <ul>
                                            <span onclick={ctx.link().callback(|_| Msg::SetCurrentTab)} 
                                                class={if self.current_tab == Some(MainRoute::Live) || self.current_tab == Some(MainRoute::Home) {"current_active_page"} else {""}}>
                                                <Link<MainRoute> to={MainRoute::Live} 
                                                    classes={"inline-block bg-blue font-bold py-1 px-5 lg:px-9 rounded-t-lg mx-2 transition-all hover:bg-white hover:text-black"}>
                                                    { "Live" }
                                                </Link<MainRoute>>
                                            </span>
                                            <span onclick={ctx.link().callback(|_| Msg::SetCurrentTab)} 
                                                class={if self.current_tab == Some(MainRoute::Upcoming)  {"current_active_page"} else {""}}>
                                                <Link<MainRoute> to={MainRoute::Upcoming} 
                                                    classes="inline-block bg-blue font-bold py-1 px-5 lg:px-9 rounded-t-lg mx-2 transition-all hover:bg-white hover:text-black">
                                                    { "Upcoming" }
                                                </Link<MainRoute>>
                                            </span>
                                            <span onclick={ctx.link().callback(|_| Msg::SetCurrentTab)} 
                                                class={if self.current_tab == Some(MainRoute::Results) {"current_active_page"} else {""}}>
                                                <Link<MainRoute> to={MainRoute::Results} 
                                                    classes="inline-block bg-blue font-bold py-1 px-5 lg:px-9 rounded-t-lg mx-2 transition-all hover:bg-white hover:text-black">
                                                    { "Results" }
                                                </Link<MainRoute>>
                                            </span>
                                        </ul>
                                    </nav>
                                }
                            }
                            _ => html!{} 
                        }
                        
                    }
                </div>
                <div class="my-auto text-sm p-2">
                    {
                        if self.user.is_authenticated() {
                            html! { 
                                <UserSummary
                                    is_admin={self.user.id == 0}
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
