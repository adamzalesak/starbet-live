use crate::{
    store::UserStore,
    types::{
        grpc_types::ticket::{ticket_service_client, ListTicketsReply, ListTicketsRequest, Ticket},
        SubmitResult,
    },
};
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
    submit_result: SubmitResult,
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
            submit_result: SubmitResult::None,
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
                    if self.tickets.is_empty() {
                        html! {
                            <div class="text-center my-2 p-1 rounded-md bg-dark-blue text-white">{"No tickets to show"}</div>
                        }
                    } else {
                        self.tickets.clone().into_iter().map(|ticket| {
                            html! {
                                <div>
                                    <div>{ticket.submitted_at}</div>
                                    <div>{ticket.price_paid}</div>
                                    <div>{match ticket.won {
                                        Some(v) => v.to_string(),
                                        None => "not evaluated yet".to_string(),
                                    }}</div>
                                </div>
                            }
                        }).collect::<Html>()
                    }
                }
            </>
        }
    }
}
