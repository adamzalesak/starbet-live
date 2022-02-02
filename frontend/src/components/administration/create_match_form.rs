use crate::types::grpc_types::game_match::{
    match_service_client, CreateMatchReply, CreateMatchRequest,
};
use crate::{
    components::{
        auth::{
            input::{InputType, TextInput},
            input_number::{NumberInput, NumberType},
        },
        loading_animation::LoadingAnimation,
    },
    types::{CreateMatchFormData, Field, SubmitResult},
};
use anyhow;
use chrono::{DateTime, Utc};
use gloo_timers::callback::Timeout;
use log::warn;
use serde::{Deserialize, Serialize};
use yew::prelude::*;

pub enum Msg {
    Submit,
    SetLoading(bool),
    SetGameId((f32, bool)),
    SetTeam1Id((f32, bool)),
    SetTeam2Id((f32, bool)),
    SetTeam1Ratio((f32, bool)),
    SetTeam2Ratio((f32, bool)),
    SetStartAt((String, Field, bool)),
    ResetSubmitResult,
    ReceiveResponse(anyhow::Result<CreateMatchReply>),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CreateMatchForm {
    is_loading: bool,
    data: CreateMatchFormData,
    submit_result: SubmitResult,
}

impl Component for CreateMatchForm {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            is_loading: false,
            submit_result: SubmitResult::None,
            data: CreateMatchFormData::new(),
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

                let datetime = match DateTime::parse_from_rfc3339(&format!(
                    "{}:00+01:00",
                    self.data.supposed_start_at.0
                )) {
                    Ok(val) => val,
                    _ => {
                        warn!("Inserted date is not valid");
                        return false;
                    }
                };
                let datetime_utc = datetime.with_timezone(&Utc);

                let grpc_client =
                    match_service_client::MatchService::new(String::from("http://127.0.0.1:5430"));
                let game_id = self.data.game_id.0 as i32;
                let team_one_id = self.data.team_one_id.0 as i32;
                let team_two_id = self.data.team_two_id.0 as i32;
                let team_one_ratio = self.data.team_one_ratio.0.to_string();
                let team_two_ratio = self.data.team_two_ratio.0.to_string();
                let supposed_start_at = datetime_utc.to_string();

                ctx.link().send_future(async move {
                    Msg::ReceiveResponse(
                        grpc_client
                            .create_match(CreateMatchRequest {
                                game_id,
                                team_one_id,
                                team_two_id,
                                team_one_ratio,
                                team_two_ratio,
                                supposed_start_at,
                                state: "unknown".to_string(),
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
            Msg::SetGameId((new_data, is_valid)) => {
                self.data.game_id = (new_data, is_valid);
                false
            }
            Msg::SetTeam1Id((new_data, is_valid)) => {
                self.data.team_one_id = (new_data, is_valid);
                false
            }
            Msg::SetTeam2Id((new_data, is_valid)) => {
                self.data.team_two_id = (new_data, is_valid);
                false
            }
            Msg::SetTeam1Ratio((new_data, is_valid)) => {
                self.data.team_one_ratio = (new_data, is_valid);
                false
            }
            Msg::SetTeam2Ratio((new_data, is_valid)) => {
                self.data.team_two_ratio = (new_data, is_valid);
                false
            }
            Msg::SetStartAt((new_data, _, is_valid)) => {
                self.data.supposed_start_at = (new_data, is_valid);
                false
            }
            Msg::ReceiveResponse(Ok(_)) => {
                self.submit_result = SubmitResult::Success;
                let link = ctx.link().clone();
                Timeout::new(5000, move || link.send_message(Msg::ResetSubmitResult)).forget();
                true
            }
            Msg::ReceiveResponse(Err(err)) => {
                warn!("{}", err.to_string());
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
                <div class="text-center font-bold text-lg">{"Create match"}</div>
                <form onsubmit={ ctx.link().callback(|e: FocusEvent| { e.prevent_default(); Msg::Submit }) }
                        class="flex flex-col gap-1 text-black admin-form">

                    <NumberInput
                        number_type={NumberType::Id}
                        label="Game ID"
                        placeholder="8"
                        on_change={ctx.link().callback(Msg::SetGameId)}
                    />
                    <div class="grid grid-cols-2 gap-2">
                        <div>
                            <NumberInput
                                number_type={NumberType::Id}
                                label="Team 1 ID"
                                placeholder="28"
                                on_change={ctx.link().callback(Msg::SetTeam1Id)}
                            />
                            <NumberInput
                                number_type={NumberType::Ratio}
                                label="Team 1 Ratio"
                                placeholder="1.34"
                                on_change={ctx.link().callback(Msg::SetTeam1Ratio)}
                            />
                        </div>
                        <div>
                            <NumberInput
                                number_type={NumberType::Id}
                                label="Team 2 ID"
                                placeholder="95"
                                on_change={ctx.link().callback(Msg::SetTeam2Id)}
                            />
                            <NumberInput
                                number_type={NumberType::Ratio}
                                label="Team 2 Ratio"
                                placeholder="2.19"
                                on_change={ctx.link().callback(Msg::SetTeam2Ratio)}
                            />
                        </div>
                    </div>
                    <TextInput
                        input_type={InputType::DateTime}
                        field={Field::Country} // ignore it, just for id
                        label="Supposed start"
                        placeholder=""
                        on_change={ctx.link().callback(Msg::SetStartAt)}
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
                                    {"Match successfully created"}
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
                        {"Create match"}
                    </button>
                </form>

            </div>
        }
    }
}
