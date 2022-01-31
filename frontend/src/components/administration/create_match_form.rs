use crate::{
    components::auth::input::{InputType, TextInput},
    types::{CreateMatchFormData, Field, SubmitResult},
};
use gloo_timers::callback::Timeout;
use log::{warn};
use serde::{Deserialize, Serialize};
use yew::prelude::*;

pub mod game_match {
    include!(concat!(env!("OUT_DIR"), concat!("/game_match.rs")));
}
use game_match::{match_service_client, CreateMatchReply, CreateMatchRequest, GameEventType};

pub enum Msg {
    Submit,
    SetGameId((u32, Field, bool)),
    SetTeam1Id((u32, Field, bool)),
    SetTeam2Id((u32, Field, bool)),
    SetTeam1Ratio((String, Field, bool)),
    SetTeam2Ratio((String, Field, bool)),
    SetStartAt((String, Field, bool)),

    ResetSubmitResult,
    ReceiveResponse(Result<CreateMatchReply, Box<dyn std::error::Error>>),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CreateMatchForm {
    data: CreateMatchFormData,
    submit_result: SubmitResult,
}

impl Component for CreateMatchForm {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
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

                let grpc_client =
                    match_service_client::MatchService::new(String::from("http://127.0.0.1:5430"));
                let game_id = self.data.game_id.0;
                let team_one_id = self.data.team_one_id.0;
                let team_two_id = self.data.team_two_id.0;
                let team_one_ratio = self.data.team_one_ratio.0.clone();
                let team_two_ratio = self.data.team_two_ratio.0.clone();
                let supposed_start_at = self.data.supposed_start_at.0.clone();

                ctx.link().send_future(async move {
                    Msg::ReceiveResponse(
                        grpc_client
                            .create_team(CreateMatchRequest {
                                game_id,
                                team_one_id,
                                team_two_id,
                                team_one_ratio,
                                team_two_ratio,
                                supposed_start_at,
                                state: "UPCOMING".to_string(),
                            })
                            .await,
                    )
                });
                true
            }
            Msg::SetGameId((new_data, _, is_valid)) => {
                self.data.game_id = (new_data, is_valid);
                false
            }
            Msg::SetTeam1Id((new_data, _, is_valid)) => {
                self.data.team_one_id = (new_data, is_valid);
                false
            }
            Msg::SetTeam2Id((new_data, _, is_valid)) => {
                self.data.team_two_id = (new_data, is_valid);
                false
            }
            Msg::SetTeam1Ratio((new_data, _, is_valid)) => {
                self.data.team_one_ratio = (new_data, is_valid);
                false
            }
            Msg::SetTeam2Ratio((new_data, _, is_valid)) => {
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
                <div class="text-center font-bold text-lg">{"Create match"}</div>
                <form onsubmit={ ctx.link().callback(|e: FocusEvent| { e.prevent_default(); Msg::Submit }) }
                        class="flex flex-col gap-1 text-black admin-form">
                    <TextInput
                        field={Field::Area} // ignore it, just for id
                        label="Game ID"
                        placeholder="8"
                        on_change={ctx.link().callback(Msg::SetTeam1Id)}
                    />
                    <TextInput
                        field={Field::StreetName} // ignore it, just for id
                        label="Team 1 ID"
                        placeholder="28"
                        on_change={ctx.link().callback(Msg::SetTeam1Id)}
                    />
                    <TextInput
                        field={Field::StreetNumber} // ignore it, just for id
                        label="Team 2 ID"
                        placeholder="95"
                        on_change={ctx.link().callback(Msg::SetTeam2Id)}
                    />
                    <TextInput
                        field={Field::City} // ignore it, just for id
                        label="Team 1 Ratio"
                        placeholder="1.34"
                        on_change={ctx.link().callback(Msg::SetTeam1Ratio)}
                    />
                    <TextInput
                        field={Field::PostalCode} // ignore it, just for id
                        label="Team 2 Ratio"
                        placeholder="2.19"
                        on_change={ctx.link().callback(Msg::SetTeam2Ratio)}
                    />
                    <TextInput
                        input_type={InputType::DateTime}
                        field={Field::Country} // ignore it, just for id
                        label="Supposed start"
                        placeholder="https://logos-download.com/wp-content/uploads/2016/06/Fnatic_logo_wordmark.png"
                        on_change={ctx.link().callback(Msg::SetStartAt)}
                    />
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
