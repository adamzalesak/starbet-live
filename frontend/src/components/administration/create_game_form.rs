use crate::{
    components::{auth::input::TextInput, loading_animation::LoadingAnimation},
    types::{
        grpc_types::game::{game_service_client, CreateGameReply, CreateGameRequest},
        CreateGameFormData, Field, SubmitResult,
    },
};
use anyhow;
use gloo_timers::callback::Timeout;
use log::{error, warn};
use serde::{Deserialize, Serialize};
use yew::prelude::*;

pub enum Msg {
    Submit,
    SetLoading(bool),
    SetName((String, Field, bool)),
    SetLogoUrl((String, Field, bool)),
    ResetSubmitResult,
    ReceiveResponse(anyhow::Result<CreateGameReply>),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CreateGameForm {
    is_loading: bool,
    data: CreateGameFormData,
    submit_result: SubmitResult,
}

impl Component for CreateGameForm {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            is_loading: false,
            submit_result: SubmitResult::None,
            data: CreateGameFormData::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Submit => {
                if !self.data.is_valid() {
                    warn!("Inserted data are not valid");
                    return false;
                }
                ctx.link().send_message(Msg::SetLoading(true));

                let grpc_client =
                    game_service_client::GameService::new(String::from("http://127.0.0.1:5430"));
                let name = self.data.name.0.trim().to_string();
                let logo_url = self.data.logo_url.0.trim().to_string();

                ctx.link().send_future(async move {
                    Msg::ReceiveResponse(
                        grpc_client
                            .create_game(CreateGameRequest { name, logo_url })
                            .await,
                    )
                });

                ctx.link().send_message(Msg::SetLoading(false));
                true
            }
            Msg::SetLoading(val) => {
                self.is_loading = val;
                true
            }
            Msg::SetName((new_data, _, is_valid)) => {
                self.data.name = (new_data, is_valid);
                false
            }
            Msg::SetLogoUrl((new_data, _, is_valid)) => {
                self.data.logo_url = (new_data, is_valid);
                false
            }
            Msg::ReceiveResponse(Ok(_)) => {
                self.submit_result = SubmitResult::Success;
                let link = ctx.link().clone();
                Timeout::new(5000, move || link.send_message(Msg::ResetSubmitResult)).forget();
                true
            }
            Msg::ReceiveResponse(Err(err)) => {
                error!("{}", err.to_string());
                self.submit_result = SubmitResult::Error;
                let link = ctx.link().clone();
                Timeout::new(5000, move || link.send_message(Msg::ResetSubmitResult)).forget();
                true
            }
            Msg::ResetSubmitResult => {
                self.submit_result = SubmitResult::None;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="bg-light-grey p-2 rounded-md mb-2">
                <div class="text-center font-bold text-lg">{"Create game"}</div>
                <form onsubmit={ ctx.link().callback(|e: FocusEvent| { e.prevent_default(); Msg::Submit }) }
                        class="flex flex-col gap-1 text-black admin-form">
                    <TextInput
                        field={Field::FirstName} // ignore it, just for id
                        label="Name"
                        placeholder="Counter-Strike: Global Offensive"
                        on_change={ctx.link().callback(Msg::SetName)}
                    />
                    <TextInput
                        field={Field::LastName} // ignore it, just for id
                        label="Logo Url"
                        placeholder="https://logos-download.com/wp-content/uploads/2016/04/Counter_Strike_logo-700x700.png"
                        on_change={ctx.link().callback(Msg::SetLogoUrl)}
                    />
                    {
                        if self.is_loading {
                            html! { <LoadingAnimation color="dark-blue" /> }
                        } else {
                            html! { }
                        }
                    }
                    {
                        if self.submit_result == SubmitResult::Success {
                            html! {
                                <div class="mx-auto my-1 p-1 w-full lg:w-9/12 text-center bg-success-light text-success rounded-md transition-all">
                                    {"Game successfully created"}
                                </div>
                            }
                        } else if self.submit_result == SubmitResult::Error {
                            html! {
                                <div class="mx-auto my-1 p-1 w-full lg:w-9/12 text-center bg-danger-light text-danger rounded-md  transition-all">
                                    { "Something went wrong :( check console for error message" }
                                </div>
                            }
                        } else {
                            html! {}
                        }
                    }
                    <button type="submit"
                            class="block w-6/12 mx-auto p-1 bg-blue text-white uppercase font-light rounded-md transition-all">
                        {"Create game"}
                    </button>
                </form>

            </div>
        }
    }
}
