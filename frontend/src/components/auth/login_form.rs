use crate::{
    components::auth::input::{InputType, TextInput},
    store::{UserRequest, UserStore},
    types::{
        grpc_types::user::{user_service_client, Address, AuthUserReply, AuthUserRequest, User},
        Field, MainRoute, SubmitResult, UserLoginFormData,
    },
};
use gloo_timers::callback::Timeout;
use log::{error, info, warn};
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_agent::{
    utils::store::{Bridgeable, ReadOnly, StoreWrapper},
    Bridge,
};
use yew_router::prelude::Link;

pub struct LoginForm {
    data: UserLoginFormData,
    error: String,
    user_store: Box<dyn Bridge<StoreWrapper<UserStore>>>,
}

pub enum Msg {
    Submit,
    SetEmail((String, Field, bool)),
    SetPassword((String, Field, bool)),
    UserStore(ReadOnly<UserStore>),
    ResetError,
    ReceiveResponse(anyhow::Result<AuthUserReply>),
}

impl Component for LoginForm {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            data: UserLoginFormData::new(),
            error: String::new(),
            user_store: UserStore::bridge(ctx.link().callback(Msg::UserStore)),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Submit => {
                if self.data.email.is_empty() || self.data.password.is_empty() {
                    warn!("Inserted login data are not valid");
                    return false;
                }
                let grpc_client =
                    user_service_client::UserService::new(String::from("http://127.0.0.1:5430"));
                let email = self.data.email.trim().to_string();
                let password = self.data.password.trim().to_string();

                ctx.link().send_future(async move {
                    Msg::ReceiveResponse(
                        grpc_client
                            .auth_user(AuthUserRequest { email, password })
                            .await,
                    )
                });
            }
            Msg::SetEmail((new_data, _, _)) => self.data.email = new_data,
            Msg::SetPassword((new_data, _, _)) => self.data.password = new_data,
            Msg::UserStore(_) => {}
            Msg::ReceiveResponse(Ok(user)) => {
                self.user_store.send(UserRequest::Login(user.user));
            }
            Msg::ReceiveResponse(Err(err)) => {
                self.error = err.to_string();
                let link = ctx.link().clone();
                Timeout::new(5000, move || link.send_message(Msg::ResetError)).forget();
            }
            Msg::ResetError => {
                self.error = String::new();
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <form onsubmit={ ctx.link().callback(|e: FocusEvent| { e.prevent_default(); Msg::Submit }) }
                        class="flex flex-row gap-1 text-black login relative" >
                    <div class="flex flex-col gap-1 lg:flex-row">
                        <TextInput
                            input_type={InputType::Email}
                            field={Field::Email}
                            label="Email address"
                            placeholder="EMAIL"
                            // value={self.data.email.clone()}
                            on_change={ctx.link().callback(Msg::SetEmail)}
                            required={false}
                        />
                        <TextInput
                            input_type={InputType::Password}
                            field={Field::Password}
                            label="Password"
                            placeholder="PASSWORD"
                            // value={self.data.password.clone()}
                            on_change={ctx.link().callback(Msg::SetPassword)}
                            required={false}
                        />
                    </div>
                    <div class="flex flex-row gap-1">
                        <button type="submit"
                                class="block my-auto px-1 py-4 lg:py-1 bg-yellow rounded-md uppercase font-light text-black transition-all">
                            {"Login"}
                        </button>
                        <Link<MainRoute> to={MainRoute::Registration}
                                classes="block my-auto px-1 py-4 lg:py-1 bg-light-grey uppercase font-light rounded-md transition-all">
                            { "Join" }
                        </Link<MainRoute>>
                    </div>
                </form>
                {
                    if !self.error.is_empty() {
                        html! {
                            <div class="px-1 my-1 rounded-md text-center text-danger bg-danger-light">
                                {self.error.clone()}
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }

            </>
        }
    }
}
