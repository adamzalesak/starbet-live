use yew::prelude::*;

pub mod game {
    include!(concat!(env!("OUT_DIR"), concat!("/game.rs")));
}

use game::{game_service_client, Game, ListGamesReply, ListGamesRequest};

pub enum Msg {
    Call,
    ReceiveResponse(Result<ListGamesReply, Box<dyn std::error::Error>>),
}

pub struct Games {
    // games: Vec<Game>,
}

impl Component for Games {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().callback(|_| Msg::Call);
        Self { games: Vec::new() }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Call => {
                let grpc_client =
                    game_service_client::GameService::new(String::from("http://127.0.0.1:50051"));
                ctx.link().send_future(async move {
                    Msg::ReceiveResponse(grpc_client.list_games(ListGamesRequest {}).await)
                });
                false
            }
            Msg::ReceiveResponse(Ok(result)) => {
                self.games = result.games;
                true
            }
            Msg::ReceiveResponse(Err(_error)) => {
                self.games = Vec::new();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
         <div class="bg-dark-blue text-white flex-auto rounded-md p-2 text-center">
            <div class="font-bold mb-2">{"Games"}</div>
            <ul class="flex flex-col gap-1.5">
                {
                    self.games.clone().into_iter().map(|game| {
                        html! {
                            <li key={game.id} class="text-black font-bold rounded-md bg-white p-1 text-left cursor-pointer">
                                { game.name }
                            </li>
                        }
                    }).collect::<Html>()
                }
            </ul>
            {self.games.len()}
            <br />
            <button class="bg-yellow p-1" onclick={ctx.link().callback(|_| Msg::Call)}>
                { "Load games" }
            </button>
        </div>
        }
    }
}
