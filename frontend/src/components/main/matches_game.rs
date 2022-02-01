use yew::prelude::*;
use crate::components::loading_animation::LoadingAnimation;

use super::match_item::MatchItem;

pub mod game_match {
    include!(concat!(env!("OUT_DIR"), concat!("/game_match.rs")));
}
pub mod team {
    include!(concat!(env!("OUT_DIR"), concat!("/team.rs")));
}
use game_match::{
    match_service_client, GameEventType, ListMatchesReply, ListMatchesRequest, Match,
};

pub enum Msg {
    Fetch,
    ReceiveResponse(Result<ListMatchesReply, Box<dyn std::error::Error>>),
}

pub struct MatchesGame {
    id: i32,
    name: String,
    logo_url: String,

    matches: Vec<Match>,
    is_loading: bool,
    is_error: bool,
}

#[derive(Properties, PartialEq)]
pub struct MatchesGameProps {
    pub id: i32,
    pub name: String,
    pub logo_url: String,
}

impl Component for MatchesGame {
    type Message = Msg;
    type Properties = MatchesGameProps;

    fn create(ctx: &Context<Self>) -> Self {
        let MatchesGameProps { id, name, logo_url } = ctx.props().clone();

        // todo uncomment after error fix
        // ctx.link().send_message(Msg::Fetch);

        Self {
            id: id.clone(),
            name: name.clone(),
            logo_url: logo_url.clone(),

            matches: Vec::new(),
            is_loading: false,
            is_error: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Fetch => {
                self.is_loading = true;
                self.is_error = false;
                let grpc_client =
                    match_service_client::MatchService::new(String::from("http://127.0.0.1:5430"));

                let game_id = self.id.clone();
                ctx.link().send_future(async move {
                    Msg::ReceiveResponse(
                        grpc_client
                            .list_matches(ListMatchesRequest {
                                game_id: game_id,
                                game_event_type: GameEventType::Live as i32,
                            })
                            .await,
                    )
                });
            }
            Msg::ReceiveResponse(Ok(response)) => {
                self.matches = response.game_matches;
                self.is_loading = false;
                self.is_error = false;
            }
            Msg::ReceiveResponse(Err(_)) => {
                self.matches = Vec::new();
                self.is_loading = false;
                self.is_error = true;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <li>
                <div class="flex gap-2 px-3 py-1 text-white font-bold bg-grey rounded-t-md">
                    if self.logo_url != "" {
                        <div class="w-6 h-6 my-auto">
                            <img src={self.logo_url.clone()} class="w-full" alt={self.name.clone()} />
                        </div>
                    }
                    {self.name.clone()}
                </div>
                <ul class="p-2 bg-light-grey rounded-b-md flex flex-col gap-2">
                    if self.is_loading {
                        <LoadingAnimation color="dark-blue" />
                    } else if self.is_error {
                        <h1>{"error"}</h1>
                    } else {{
                        self.matches.clone().into_iter().map(|match_item| {
                            let match_id = match_item.id.clone();
                            html! {
                                <li key={ match_id }>
                                    <MatchItem />
                                </li>
                            }
                        }).collect::<Html>()
                    }}
                </ul>
            </li>
        }
    }
}
