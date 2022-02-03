use crate::{
    store::{UserRequest, UserStore},
    types::{grpc_types::user::User, MainRoute, ProfileRoute},
};
use yew::prelude::*;
use yew_agent::{
    utils::store::{Bridgeable, ReadOnly, StoreWrapper},
    Bridge,
};
use yew_router::{
    history::History,
    prelude::{Link, RouterScopeExt},
};

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
    user: Option<User>,
    user_store: Box<dyn Bridge<StoreWrapper<UserStore>>>,
}

impl Component for LayoutProfile {
    type Message = Msg;
    type Properties = LayoutProfileProps;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            current_tab: ctx.link().route::<ProfileRoute>(),
            user: None,
            user_store: UserStore::bridge(ctx.link().callback(Msg::UserStore)),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UserStore(state) => {
                let state = state.borrow();
                self.user = state.user.clone();

                // only authenticated user can access profile page
                match state.user {
                    None => {
                        let history = ctx.link().history().unwrap();
                        history.push(MainRoute::Home);
                    }
                    Some(_) => {}
                }
            }
            Msg::SetCurrentTab => {
                self.current_tab = ctx.link().route::<ProfileRoute>();
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let (name, last_name, balance) = match &self.user {
            Some(val) => (
                val.first_name.clone(),
                val.last_name.clone(),
                val.balance.clone(),
            ),
            None => (String::new(), String::new(), String::new()),
        };
        html! {
            <>
                <div class="relative">
                    <picture class="transition-all">
                        <source media="(min-width:1024px)" srcset="/profile-background-cropped.jpg" />
                        <img src="/profile-background.jpg" alt="profile background image" style="width:auto;" />
                    </picture>

                    <div class="absolute flex flex-row gap-3 content-center p-4 rounded-xl text-dark-blue blur-background">
                        <div class="w-28">
                            <img src="/user.svg" alt="user profile picture" class="min-w-full"/>
                        </div>
                        <div class="my-auto flex flex-col">
                            <span class="text-xl font-bold">{format!("{} {}", name, last_name)}</span>
                            <span>{format!("Balance: {}€", balance)}</span>
                        </div>
                    </div>
                </div>

                <div class="grid grid-cols-3 text-center profile-nav">
                    <div onclick={ ctx.link().callback(|_| Msg::SetCurrentTab) } class={format!("font-medium transition-all {}",
                                                                                        if self.current_tab == Some(ProfileRoute::Summary)
                                                                                            {"bg-light-grey"} else {"bg-blue text-white"})}>
                        <Link<ProfileRoute> to={ProfileRoute::Summary} classes="block p-1">
                            { "Summary" }
                        </Link<ProfileRoute>>
                    </div>
                    <div onclick={ ctx.link().callback(|_| Msg::SetCurrentTab) } class={format!("font-medium transition-all {}",
                                                                                        if self.current_tab == Some(ProfileRoute::Tickets)
                                                                                            {"bg-light-grey"} else {"bg-blue text-white"})}>
                        <Link<ProfileRoute> to={ProfileRoute::Tickets} classes="blockp-1 ">
                            { "Tickets" }
                        </Link<ProfileRoute>>
                    </div>
                    <div onclick={ ctx.link().callback(|_| Msg::SetCurrentTab) } class={format!("font-medium transition-all {}",
                                                                                        if self.current_tab == Some(ProfileRoute::Statistics)
                                                                                            {"bg-light-grey"} else {"bg-blue text-white"})}>
                        <Link<ProfileRoute> to={ProfileRoute::Statistics} classes="blockp-1 ">
                            { "Statistics" }
                        </Link<ProfileRoute>>
                    </div>
                </div>
                { ctx.props().children.clone() }

            </>
        }
    }
}
