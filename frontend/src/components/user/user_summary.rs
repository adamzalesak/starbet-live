use crate::Route;
use yew::prelude::*;
use yew_router::prelude::Link;

pub enum Msg {}

pub struct UserSummary {}

#[derive(Clone, PartialEq, Properties)]
pub struct UserSummaryProps {
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
            first_name,
            last_name,
            current_balance,
        } = ctx.props().clone();

        html! {
            // <div class={"bg-dark-blue text-white text-center text-sm p-2"}>
            <div class="flex flex-row gap-2 p-3 rounded-md bg-light-grey whitespace-nowrap user-summary">
                <Link<Route> to={Route::Registration} classes="block p-2 my-auto bg-blue rounded-md transition-all">
                    { "My tickets" }
                </Link<Route>>

                <Link<Route> to={Route::Profile} classes="block p-2 my-auto bg-blue rounded-md transition-all">
                    { "Profile" }
                </Link<Route>>

                <div class="my-auto text-right text-black">
                    <span>{ first_name }{" "}{ last_name }</span>
                    <div>
                        <span class="text-xs">{"Balance: "}</span>
                        <span>{ current_balance }{"€"}</span>
                    </div>
                </div>
                <div class="w-10">
                    <img src="/user.svg" alt="user profile picture"/>
                </div>
            </div>
        }
    }
}
