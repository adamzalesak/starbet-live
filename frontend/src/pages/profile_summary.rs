use crate::{store::UserStore, types::grpc_types::user::User};
use log::{error, info};
use yew::prelude::*;
use yew_agent::{
    utils::store::{Bridgeable, ReadOnly, StoreWrapper},
    Bridge,
};

pub enum Msg {
    SetLoading(bool),
    UserStore(ReadOnly<UserStore>),
}

pub struct ProfileSummary {
    is_loading: bool,
    user: Option<User>,
    error: String,
    user_store: Box<dyn Bridge<StoreWrapper<UserStore>>>,
}

impl Component for ProfileSummary {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            user: None,
            user_store: UserStore::bridge(ctx.link().callback(Msg::UserStore)),
            error: String::new(),
            is_loading: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetLoading(val) => {
                self.is_loading = val;
                true
            }
            Msg::UserStore(state) => {
                let state = state.borrow();
                self.user = match &state.user {
                    Some(val) => Some(val.clone()),
                    None => None,
                };
                false
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
            </>
        }
    }
}
