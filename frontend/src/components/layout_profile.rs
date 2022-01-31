use crate::types::{MainRoute, ProfileRoute};
use crate::{store::UserStore, types::UserInfo};
use yew::prelude::*;
use yew_agent::{
    utils::store::{Bridgeable, ReadOnly, StoreWrapper},
    Bridge,
};
use yew_router::history::History;
use yew_router::prelude::{Link, RouterScopeExt};

pub enum Msg {
    SetCurrentTab,
    UserStore(ReadOnly<UserStore>),
}

#[derive(Properties, PartialEq)]
pub struct LayoutProfileProps {
    #[prop_or_default]
    pub children: Children,
}

pub struct LayoutProfile {
    current_tab: Option<ProfileRoute>,
    user: UserInfo,
    user_store: Box<dyn Bridge<StoreWrapper<UserStore>>>,
}

impl Component for LayoutProfile {
    type Message = Msg;
    type Properties = LayoutProfileProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            current_tab: ctx.link().route::<ProfileRoute>(),
            user: UserInfo::new(),
            user_store: UserStore::bridge(ctx.link().callback(Msg::UserStore)),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UserStore(state) => {
                let state = state.borrow();
                self.user = state.user.clone();

                // only authenticated user can access profile page
                if !self.user.is_authenticated() {
                    let history = ctx.link().history().unwrap();
                    history.push(MainRoute::Home);
                }
            }
            Msg::SetCurrentTab => {
                self.current_tab = ctx.link().route::<ProfileRoute>();
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <span onclick={ ctx.link().callback(|_| Msg::SetCurrentTab) } class={if self.current_tab == Some(ProfileRoute::Summary) {"bg-blue text-white"} else {""}}>
                    <Link<ProfileRoute> to={ProfileRoute::Summary}>
                        { "Summary" }
                    </Link<ProfileRoute>>
                </span>
                <span onclick={ ctx.link().callback(|_| Msg::SetCurrentTab) } class={if self.current_tab == Some(ProfileRoute::Tickets) {"bg-blue text-white"} else {""}}>
                    <Link<ProfileRoute> to={ProfileRoute::Tickets}>
                        { "Tickets" }
                    </Link<ProfileRoute>>
                </span>
                <span onclick={ ctx.link().callback(|_| Msg::SetCurrentTab) } class={if self.current_tab == Some(ProfileRoute::Statistics) {"bg-blue text-white"} else {""}}>
                    <Link<ProfileRoute> to={ProfileRoute::Statistics}>
                        { "Statistics" }
                    </Link<ProfileRoute>>
                </span>
                { ctx.props().children.clone() }

            </>
        }
    }
}
