use crate::{
    components::auth::input::{InputType, TextInput},
    types::{CreateGameFormData, CreateTeamFormData, Field, MainRoute, SubmitResult},
};
use gloo_timers::callback::Timeout;
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::time::Duration;
use yew::prelude::*;
use yew_router::prelude::Link;

pub mod team {
    include!(concat!(env!("OUT_DIR"), concat!("/team.rs")));
}
use team::{team_service_client, CreateTeamReply, CreateTeamRequest};

pub enum Msg {
    Submit,
    SetName((String, Field, bool)),
    SetDescription((String, Field, bool)),
    SetLogoUrl((String, Field, bool)),
    ResetSubmitResult,
    ReceiveResponse(Result<CreateTeamReply, Box<dyn std::error::Error>>),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CreateTeamForm {
    data: CreateTeamFormData,
    submit_result: SubmitResult,
}

impl Component for CreateTeamForm {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            submit_result: SubmitResult::None,
            data: CreateTeamFormData::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Submit => {
                if !self.data.is_valid() {
                    warn!("Inserted data are not valid");
                    return false;
                }

                let grpc_client =
                    team_service_client::TeamService::new(String::from("http://127.0.0.1:5430"));
                let name = self.data.name.0.clone();
                let description = self.data.description.0.clone();
                let logo = self.data.logo_url.0.clone();

                ctx.link().send_future(async move {
                    Msg::ReceiveResponse(
                        grpc_client
                            .create_team(CreateTeamRequest {
                                name,
                                description,
                                logo,
                            })
                            .await,
                    )
                });
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
            Msg::SetDescription((new_data, _, is_valid)) => {
                self.data.description = (new_data, is_valid);
                false
            }
            Msg::ReceiveResponse(Ok(_)) => {
                self.submit_result = SubmitResult::Success;
                let link = ctx.link().clone();
                Timeout::new(5000, move || link.send_message(Msg::ResetSubmitResult)).forget();
                true
            }
            Msg::ReceiveResponse(Err(_)) => {
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
            <div class="bg-light-grey p-2 rounded-md my-2">
                <div class="text-center font-bold text-lg">{"Create team"}</div>
                <form onsubmit={ ctx.link().callback(|e: FocusEvent| { e.prevent_default(); Msg::Submit }) }
                        class="flex flex-col gap-1 text-black admin-form">
                    <TextInput
                        field={Field::FirstName} // ignore it
                        label="Name"
                        placeholder="Fnatic"
                        on_change={ctx.link().callback(Msg::SetName)}
                    />
                    <TextInput
                        field={Field::LastName} // ignore it
                        label="Short description"
                        // value={self.data.logo_url.0.clone()}
                        placeholder="Fnatic is the world's leading esports organisation..."
                        on_change={ctx.link().callback(Msg::SetDescription)}
                    />
                    <TextInput
                        field={Field::LastName} // ignore it
                        label="Logo Url"
                        // value={self.data.logo_url.0.clone()}
                        placeholder="https://logos-download.com/wp-content/uploads/2016/06/Fnatic_logo_wordmark.png"
                        on_change={ctx.link().callback(Msg::SetLogoUrl)}
                    />
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
                                    {"Something went wrong :( please try again later"}
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
