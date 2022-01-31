use crate::{
    components::auth::input::{InputType, TextInput},
    types::{CreateGameFormData, Field, MainRoute},
};
use log::{info, warn};
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_router::prelude::Link;

pub mod game {
    include!(concat!(env!("OUT_DIR"), concat!("/game.rs")));
}
use game::{game_service_client, CreateGameReply, CreateGameRequest, Game};

pub enum Msg {
    Submit,
    SetName((String, Field, bool)),
    SetLogoUrl((String, Field, bool)),
    ReceiveResponse(Result<CreateGameReply, Box<dyn std::error::Error>>),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct CreateGameForm {
    data: CreateGameFormData,
    error: String,
}

impl Component for CreateGameForm {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            error: String::new(),
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
                let grpc_client =
                    game_service_client::GameService::new(String::from("http://127.0.0.1:5430"));
                ctx.link().send_future(async move {
                    Msg::ReceiveResponse(
                        grpc_client
                            .create_game(CreateGameRequest {
                                name: self.data.name.0.clone(),
                                logo_url: self.data.logo_url.0.clone(),
                            })
                            .await,
                    )
                });
                false
            }
            Msg::SetName((new_data, _, is_valid)) => {
                self.data.name = (new_data, is_valid);
                false
            }
            Msg::SetLogoUrl((new_data, _, is_valid)) => {
                self.data.logo_url = (new_data, is_valid);
                false
            }
            Msg::ReceiveResponse(Ok(result)) => true,
            Msg::ReceiveResponse(Err(_error)) => true,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <div class="bg-light-grey p-2 rounded-md">
                <div class="text-center font-bold text-lg">{"Create a game"}</div>
                <form onsubmit={ ctx.link().callback(|e: FocusEvent| { e.prevent_default(); Msg::Submit }) }
                        class="flex flex-col gap-1 text-black admin-form">
                    <TextInput
                        field={Field::FirstName}
                        label="Name"
                        placeholder="Fnatic"
                        on_change={ctx.link().callback(Msg::SetName)}
                    />
                    <TextInput
                        field={Field::LastName}
                        label="Logo Url"
                        placeholder="./pictures/fnatic_logo.png"
                        on_change={ctx.link().callback(Msg::SetLogoUrl)}
                    />
                    <button type="submit"
                            class="block w-6/12 mx-auto p-1 bg-blue text-white uppercase font-light rounded-md transition-all">
                        {"Create game"}
                    </button>
                </form>

            </div>
        }
    }
}
