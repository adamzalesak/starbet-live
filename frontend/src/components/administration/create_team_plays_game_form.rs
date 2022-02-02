use crate::{
    components::{
        auth::input_number::{NumberInput, NumberType},
        loading_animation::LoadingAnimation,
    },
    types::{
        grpc_types::team::{team_service_client, AddTeamToGameReply, AddTeamToGameRequest},
        SubmitResult,
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
    SetGameId((f32, bool)),
    SetTeamId((f32, bool)),
    ResetSubmitResult,
    ReceiveResponse(anyhow::Result<AddTeamToGameReply>),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CreateTeamPlaysGameForm {
    is_loading: bool,
    game_id: (f32, bool),
    team_id: (f32, bool),
    submit_result: SubmitResult,
}

impl Component for CreateTeamPlaysGameForm {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            is_loading: false,
            submit_result: SubmitResult::None,
            game_id: (0.0, false),
            team_id: (0.0, false),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Submit => {
                if !self.game_id.1 || !self.team_id.1 {
                    warn!("Inserted data are not valid");
                    return false;
                }
                ctx.link().send_message(Msg::SetLoading(true));

                let grpc_client =
                    team_service_client::TeamService::new(String::from("http://127.0.0.1:5430"));

                let team_id: i32 = self.team_id.0 as i32;
                let game_id: i32 = self.game_id.0 as i32;

                ctx.link().send_future(async move {
                    Msg::ReceiveResponse(
                        grpc_client
                            .add_team_to_game(AddTeamToGameRequest { team_id, game_id })
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
                self.game_id = (new_data, is_valid);
                false
            }
            Msg::SetTeamId((new_data, is_valid)) => {
                self.team_id = (new_data, is_valid);
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
                <div class="text-center font-bold text-lg">{"Create team plays game"}</div>
                <form onsubmit={ ctx.link().callback(|e: FocusEvent| { e.prevent_default(); Msg::Submit }) }
                        class="flex flex-col gap-1 text-black admin-form">
                    <div class="grid grid-cols-2 gap-2">
                        <NumberInput
                            number_type={NumberType::Id}
                            label="Team ID"
                            placeholder="28"
                            on_change={ctx.link().callback(Msg::SetTeamId)}
                        />
                        <NumberInput
                            number_type={NumberType::Id}
                            label="Game ID"
                            placeholder="7"
                            on_change={ctx.link().callback(Msg::SetGameId)}
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
                                    {"Team successfully added to game"}
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
                        {"Add team to game"}
                    </button>
                </form>

            </div>
        }
    }
}
