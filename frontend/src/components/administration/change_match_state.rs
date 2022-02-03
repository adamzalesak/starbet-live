use crate::{
    components::{
        auth::{
            input::{InputType, TextInput},
            input_number::{NumberInput, NumberType},
        },
        loading_animation::LoadingAnimation,
    },
    types::{
        grpc_types::game_match::{match_service_client, ChangeStateReply, ChangeStateRequest},
        Field, SubmitResult,
    },
};
use anyhow;
use gloo_timers::callback::Timeout;
use log::{error, warn};
use serde::{Deserialize, Serialize};
use yew::prelude::*;

#[derive(Clone, Copy, Deserialize, Serialize, PartialEq)]
enum EventType {
    Upcoming,
    Live,
    Ended,
}
pub enum Msg {
    Submit,
    SetLoading(bool),
    SetMatchId((f32, bool)),
    SetMatchState((String, Field, bool)),
    ResetSubmitResult,
    ReceiveResponse(anyhow::Result<ChangeStateReply>),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ChangeMatchState {
    is_loading: bool,
    match_id: (f32, bool),
    match_state: (String, bool),
    submit_result: SubmitResult,
}

impl Component for ChangeMatchState {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            is_loading: false,
            submit_result: SubmitResult::None,
            match_id: (0.0, false),
            match_state: (String::new(), false),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Submit => {
                if !self.match_id.1 || !self.match_state.1 {
                    warn!("Inserted data are not valid");
                    return false;
                }
                ctx.link().send_message(Msg::SetLoading(true));

                let grpc_client =
                    match_service_client::MatchService::new(String::from("http://127.0.0.1:5430"));

                let match_id = self.match_id.0 as i32;
                let state = self.match_state.0.trim().to_string();

                ctx.link().send_future(async move {
                    Msg::ReceiveResponse(
                        grpc_client
                            .change_state(ChangeStateRequest {
                                match_id,
                                state,
                            })
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
            Msg::SetMatchId((new_data, is_valid)) => {
                self.match_id = (new_data, is_valid);
                false
            }
            Msg::SetMatchState((new_data, _, is_valid)) => {
                self.match_state = (new_data, is_valid);
                false
            }
            Msg::ReceiveResponse(Ok(_)) => {
                error!("success");
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
                <div class="text-center font-bold text-lg">{"Change match state"}</div>
                <form onsubmit={ ctx.link().callback(|e: FocusEvent| { e.prevent_default(); Msg::Submit }) }
                        class="flex flex-col gap-1 text-black admin-form">
                    <div class="grid grid-cols-2 gap-2">
                        <NumberInput
                            number_type={NumberType::Id}
                            label="Match  ID"
                            placeholder="48"
                            on_change={ctx.link().callback(Msg::SetMatchId)}
                        />
                        <TextInput
                            field={Field::PasswordConfirmation} // ignore it, just for id
                            label="Match state"
                            placeholder="12 kills - 38 kills"
                            on_change={ctx.link().callback(Msg::SetMatchState)}
                        />
                    </div>
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
                                    {"Match state successfully changed"}
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
                        {"Change state"}
                    </button>
                </form>

            </div>
        }
    }
}
