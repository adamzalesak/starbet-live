use crate::types::ProfileRoute;
use yew::prelude::*;
use yew_router::prelude::Link;

pub enum Msg {}

pub struct UserSummary {}

#[derive(Clone, PartialEq, Properties)]
pub struct UserSummaryProps {
    pub on_logout: Callback<()>,
    pub is_admin: bool,
    pub first_name: String,
    pub last_name: String,
    pub current_balance: String,
}

impl Component for UserSummary {
    type Message = Msg;
    type Properties = UserSummaryProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let UserSummaryProps {
            on_logout,
            is_admin,
            first_name,
            last_name,
            current_balance,
        } = ctx.props().clone();

        html! {
            <div class="flex flex-row gap-2 p-3 rounded-md bg-light-grey whitespace-nowrap user-summary">
                {
                    if is_admin {
                        html! {
                            <Link<ProfileRoute> to={ProfileRoute::Administration} classes="block p-2 my-auto bg-blue rounded-md transition-all uppercase font-light">
                                { "Admin" }
                            </Link<ProfileRoute>>
                        }
                    } else {
                        html! {}
                    }
                }
                <Link<ProfileRoute> to={ProfileRoute::Tickets} classes="block p-2 my-auto bg-blue rounded-md transition-all uppercase font-light">
                    { "My tickets" }
                </Link<ProfileRoute>>

                <button type="button" onclick={move |_| on_logout.emit(())} class="block p-2 my-auto bg-dark-blue rounded-md transition-all uppercase font-light">
                    {"Logout"}
                </button>

                <div class="my-auto text-right text-black">
                    <span>{ first_name }{" "}{ last_name }</span>
                    <div>
                        <span class="text-xs">{"Balance: "}</span>
                        <span>{ current_balance }{"€"}</span>
                    </div>
                </div>

                <div class="w-10 h-10">
                    <img src="/user.svg" alt="user profile picture" class="min-w-full"/>
                </div>
            </div>
        }
    }
}
