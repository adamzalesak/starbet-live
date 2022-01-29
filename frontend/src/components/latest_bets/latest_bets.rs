use gloo::console::info;
use wasm_sockets::{self, Message, WebSocketError};
use yew::prelude::*;

pub enum Msg {
    Connect,
    ReceiveBets(String),
}

pub struct LatestBets {
    client: Option<wasm_sockets::EventClient>,
    latest_tickets: Vec<String>,
}

impl Component for LatestBets {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        // ctx.link().send_message(Msg::Connect);

        Self {
            latest_tickets: Vec::new(),
            client: None,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        info!("updated");
        match msg {
            Msg::Connect => {
                let mut client = wasm_sockets::EventClient::new("ws://127.0.0.1:8000/bet").unwrap();

                let callback = ctx.link().callback(|text: String| Msg::ReceiveBets(text));

                client.set_on_message(Some(Box::new(
                    move |_: &wasm_sockets::EventClient, message: wasm_sockets::Message| {
                        // replace to Message::Binary
                        if let Message::Text(text) = message {
                            callback.emit(text);
                        };
                    },
                )));

                self.client = Some(client);

                true
            }
            Msg::ReceiveBets(text) => {
                self.latest_tickets.push(text);
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
         <div class="bg-dark-blue text-white rounded-md p-2 h-2/6">
            <div class="h-1/5">
                <div class="font-bold text-center">{"Latest bets"}</div>
                <button class="btn btn-primary" onclick={ ctx.link().callback(|_| Msg::Connect) }>{"Connect"}</button>
            </div>
            <div class="h-4/5">
                <ul class="h-50 flex flex-col gap-1.5 overflow-y-auto max-h-full">
                    {
                        self.latest_tickets.clone().into_iter().map(|ticket| {
                            html! {
                                <li key={ticket.clone()} class="text-black font-bold rounded-md bg-white p-1 text-left cursor-pointer">
                                    { ticket }
                                </li>
                            }
                        }).collect::<Html>()
                    }
                </ul>
            </div>
         </div>
        }
    }
}
