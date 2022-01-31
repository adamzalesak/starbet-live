use yew::prelude::*;

pub mod game {
    include!(concat!(env!("OUT_DIR"), concat!("/game.rs")));
}

use game::{game_service_client, CreateGameRequest, Game, ListGamesReply, ListGamesRequest};

pub enum Msg {
    FetchGames,
    ReceiveResponse(Result<ListGamesReply, Box<dyn std::error::Error>>),
}

pub struct Games {
    games: Vec<Game>,
    is_loading_games: bool,
    error_games: bool,
}

impl Component for Games {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::FetchGames);
        Self {
            games: Vec::new(),
            is_loading_games: true,
            error_games: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchGames => {
                self.is_loading_games = true;
                self.error_games = false;
                let grpc_client =
                    game_service_client::GameService::new(String::from("http://127.0.0.1:5430"));
                ctx.link().send_future(async move {
                    // grpc_client
                    //     .create_game(CreateGameRequest {
                    //         name: String::from("CS:GO"),
                    //         logo_url: String::from("https://logos-download.com/wp-content/uploads/2016/04/Counter_Strike_logo-700x700.png"),
                    //     })
                    //     .await;
                    Msg::ReceiveResponse(grpc_client.list_games(ListGamesRequest {}).await)
                });
                false
            }
            Msg::ReceiveResponse(Ok(result)) => {
                self.games = result.games;
                self.is_loading_games = false;
                true
            }
            Msg::ReceiveResponse(Err(_error)) => {
                self.games = Vec::new();
                self.is_loading_games = false;
                self.error_games = true;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
         <div class="bg-dark-blue text-white flex-auto rounded-md p-2 text-center">
            <div class="font-bold mb-2">{"Games"}</div>

            if self.is_loading_games {
                <h1>{"loading"}</h1>
            }
            else if self.error_games {
                <h1>{"error"}</h1>
            }
            else {
                <ul class="flex flex-col gap-1.5">
                    {
                        self.games.clone().into_iter().map(|game| {
                            html! {
                                <li key={game.id} class="flex gap-2 text-black font-bold rounded-md bg-white p-1 text-left cursor-pointer">
                                    if game.logo_url != "" {
                                        <img src={game.logo_url} class="h-6 w-6" />
                                    }
                                    { game.name }
                                </li>
                            }
                        }).collect::<Html>()
                    }
                </ul>
            }

        </div>
        }
    }
}
