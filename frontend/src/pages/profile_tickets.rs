use crate::{
    components::loading_animation::LoadingAnimation,
    store::UserStore,
    types::grpc_types::ticket::{
        ticket_service_client, ListTicketsReply, ListTicketsRequest, Ticket,
    },
};
use chrono::NaiveDateTime;
use log::{error, info};
use yew::prelude::*;
use yew_agent::{
    utils::store::{Bridgeable, ReadOnly, StoreWrapper},
    Bridge,
};

pub enum Msg {
    FetchTickets,
    SetLoading(bool),
    ReceiveResponse(anyhow::Result<ListTicketsReply>),
    UserStore(ReadOnly<UserStore>),
}

pub struct ProfileTickets {
    tickets: Vec<Ticket>,
    is_loading: bool,
    user_id: i32,
    error: String,
    user_store: Box<dyn Bridge<StoreWrapper<UserStore>>>,
}

impl Component for ProfileTickets {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            user_id: 0,
            user_store: UserStore::bridge(ctx.link().callback(Msg::UserStore)),
            tickets: Vec::new(),
            error: String::new(),
            is_loading: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::FetchTickets => {
                ctx.link().send_message(Msg::SetLoading(true));

                let grpc_client = ticket_service_client::TicketService::new(String::from(
                    "http://127.0.0.1:5430",
                ));
                let user_id = self.user_id;
                ctx.link().send_future(async move {
                    Msg::ReceiveResponse(
                        grpc_client
                            .list_tickets(ListTicketsRequest { user_id })
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
            Msg::ReceiveResponse(Ok(result)) => {
                self.tickets = result.tickets.clone();
                true
            }
            Msg::ReceiveResponse(Err(err)) => {
                self.error = err.to_string();
                true
            }
            Msg::UserStore(state) => {
                let state = state.borrow();
                self.user_id = match &state.user {
                    Some(val) => val.id,
                    _ => 0,
                };
                ctx.link().send_message(Msg::FetchTickets);
                false
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <>
                {
                    if self.is_loading {
                        html! { <LoadingAnimation color="dark-blue" /> }
                    } else {
                        html! { }
                    }
                }
                {
                    if self.tickets.is_empty() {
                        html! {
                            <div class="text-center my-2 p-1 rounded-md bg-dark-blue text-white">{"No tickets to show"}</div>
                        }
                    } else {
                        let mut temp = self.tickets.clone();
                        temp.sort_by_key(|x| x.submitted_at.clone());
                        temp.reverse();
                        temp.into_iter().map(|ticket| {
                            let price_paid = match ticket.price_paid.parse::<f32>() {
                                Ok(val) => val,
                                _ => 1.0,
                            };
                            let total_ratio = match ticket.total_ratio.parse::<f32>() {
                                Ok(val) => val,
                                _ => 1.0,
                            };

                            let temp =
                                NaiveDateTime::parse_from_str(&ticket.submitted_at, "%Y-%m-%d %H:%M:%S%.9f UTC")
                                    .unwrap();
                            let submitted_at = temp.format("%d/%m/%Y %H:%M").to_string();

                            html! {
                                <div class={format!("rounded-md p-1 mt-2 border {}", match ticket.won {
                                    Some(true) => "bg-success-light border-success".to_string(),
                                    Some(false) => "bg-danger-light border-danger".to_string(),
                                    None => "bg-light-grey".to_string(),
                                })}>
                                    <div class="grid grid-cols-2">
                                        <div>
                                            <div>{"Number of bets: "}<span class="font-bold">{ticket.bets.len()}</span></div>
                                            <div>{"Total ratio: "}<span class="font-bold">{total_ratio}</span></div>
                                        </div>
                                        <div>
                                            <div>{"Price paid: "}<span class="font-bold">{price_paid}</span>{"€"}</div>
                                            <div>{"Eventual win: "}<span class="font-bold">{total_ratio * price_paid}</span>{"€"}</div>
                                        </div>
                                    </div>
                                    <div class="text-sm border-t border-black">{format!("Submitted at: {}", submitted_at)}</div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                }
            </>
        }
    }
}
