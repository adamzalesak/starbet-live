use crate::{
    components::{
        auth::input_number::{NumberInput, NumberType},
        loading_animation::LoadingAnimation,
    },
    types::{
        grpc_types::game_match::{
            match_service_client, CreateGameEventReply, CreateGameEventRequest, GameEventType,
        },
        SubmitResult,
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
    SetEvent(EventType),
    SetMatchId((f32, bool)),
    SetWinningTeamId((f32, bool)),
    ResetSubmitResult,
    ReceiveResponse(anyhow::Result<CreateGameEventReply>),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct ChangeMatchEvent {
    is_loading: bool,
    match_id: (f32, bool),
    event_type: Option<EventType>,
    winning_team_id: Option<(f32, bool)>,
    submit_result: SubmitResult,
}

impl Component for ChangeMatchEvent {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            is_loading: false,
            submit_result: SubmitResult::None,
            event_type: None,
            match_id: (0.0, false),
            winning_team_id: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Submit => {
                if !self.match_id.1 || self.event_type == None {
                    warn!("Inserted data are not valid");
                    return false;
                }
                if self.event_type == Some(EventType::Ended) && self.winning_team_id == None {
                    warn!("Insert ID of winning team");
                    return false;
                }
                ctx.link().send_message(Msg::SetLoading(true));

                let grpc_client =
                    match_service_client::MatchService::new(String::from("http://127.0.0.1:5430"));

                let match_id = self.match_id.0 as i32;
                let game_event_type = match self.event_type {
                    Some(EventType::Upcoming) => GameEventType::Upcoming as i32,
                    Some(EventType::Live) => GameEventType::Live as i32,
                    Some(EventType::Ended) => GameEventType::Ended as i32,
                    None => return false,
                };

                ctx.link().send_future(async move {
                    Msg::ReceiveResponse(
                        grpc_client
                            .create_game_event(CreateGameEventRequest {
                                match_id,
                                game_event_type,
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
            Msg::SetEvent(val) => {
                self.event_type = Some(val);
                true
            }
            Msg::SetMatchId((new_data, is_valid)) => {
                self.match_id = (new_data, is_valid);
                false
            }
            Msg::SetWinningTeamId((new_data, is_valid)) => {
                self.winning_team_id = Some((new_data, is_valid));
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
                <div class="text-center font-bold text-lg">{"Change match event"}</div>
                <form onsubmit={ ctx.link().callback(|e: FocusEvent| { e.prevent_default(); Msg::Submit }) }
                        class="flex flex-col gap-1 text-black admin-form">
                    <NumberInput
                        number_type={NumberType::Id}
                        label="Match ID"
                        placeholder="48"
                        on_change={ctx.link().callback(Msg::SetMatchId)}
                    />
                    <div class="grid grid-cols-3 gap-2 my-2">
                        <button type="button" onclick={ctx.link().callback(|_| Msg::SetEvent(EventType::Upcoming))}
                            class={format!("my-auto rounded-md p-2 transition-all {}",
                                    if self.event_type == Some(EventType::Upcoming) {"bg-blue text-white"} else {"bg-yellow"})}>
                            {"Upcoming"}
                        </button>
                        <button type="button" onclick={ctx.link().callback(|_| Msg::SetEvent(EventType::Live))}
                            class={format!("my-auto rounded-md p-2 transition-all {}",
                                    if self.event_type == Some(EventType::Live) {"bg-blue text-white"} else {"bg-yellow"})}>
                            {"Live"}
                        </button>
                        <button type="button" onclick={ctx.link().callback(|_| Msg::SetEvent(EventType::Ended))}
                            class={format!("my-auto rounded-md p-2 transition-all {}",
                                    if self.event_type == Some(EventType::Ended) {"bg-blue text-white"} else {"bg-yellow"})}>
                            {"Ended"}
                        </button>
                    </div>
                    {
                        if self.event_type == Some(EventType::Ended) {
                            html! {
                                <NumberInput
                                    number_type={NumberType::Id}
                                    label="Winning team ID"
                                    placeholder="31"
                                    on_change={ctx.link().callback(Msg::SetWinningTeamId)}
                                />
                            }
                        } else {
                            html! {}
                        }
                    }
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
                                    {"Match event successfully changed"}
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
                        {"Change event"}
                    </button>
                </form>

            </div>
        }
    }
}
