use crate::{
    components::{
        layout::Layout, layout_no_sidebars::LayoutNoSidebars, layout_profile::LayoutProfile,
    },
    pages::{
        about_page::AboutPage, administration_page::AdministrationPage, contact_page::ContactPage,
        live_page::LivePage, not_found::NotFoundPage, privacy_policy_page::PrivacyPolicyPage,
        profile_statistics::ProfileStatistics, profile_summary::ProfileSummary,
        profile_tickets::ProfileTickets, registration_page::RegistrationPage,
        results_page::ResultsPage, upcoming_page::UpcomingPage,
    },
    store::{MatchesRequest, MatchesStore, UserRequest, UserStore},
    types::{grpc_types::game_match::Match, MainRoute, ProfileRoute},
};
use bytes::BytesMut;
use prost::{DecodeError, Message as ProstMessage};
use wasm_sockets::{self, Message, WebSocketError};
use yew::prelude::*;
use yew_agent::{
    utils::store::{Bridgeable, ReadOnly, StoreWrapper},
    Bridge,
};
use yew_router::{prelude::Redirect, BrowserRouter, Switch};

mod components;
mod pages;
mod services;
mod store;
mod types;

enum Msg {
    UserStore(ReadOnly<UserStore>),
    InitUser,
    MatchesStore(ReadOnly<MatchesStore>),
    FetchMatches,
    ReceiveMatchUpdate(Result<Match, DecodeError>),
}

struct App {
    user_store: Box<dyn Bridge<StoreWrapper<UserStore>>>,
    matches_store: Box<dyn Bridge<StoreWrapper<MatchesStore>>>,
    ws_client: wasm_sockets::EventClient,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        ctx.link().send_message(Msg::InitUser);

        let mut client = wasm_sockets::EventClient::new("ws://127.0.0.1:50052/match").unwrap();

        let callback = ctx
            .link()
            .callback(|match_item: Result<Match, DecodeError>| Msg::ReceiveMatchUpdate(match_item));

        client.set_on_message(Some(Box::new(
            move |_: &wasm_sockets::EventClient, message: wasm_sockets::Message| {
                if let Message::Binary(data) = message {
                    let mut buf = BytesMut::with_capacity(64);
                    buf.extend_from_slice(&data);
                    callback.emit(Match::decode(buf));
                };
            },
        )));

        ctx.link().send_message(Msg::FetchMatches);

        Self {
            user_store: UserStore::bridge(ctx.link().callback(Msg::UserStore)),
            matches_store: MatchesStore::bridge(ctx.link().callback(Msg::MatchesStore)),
            ws_client: client,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MatchesStore(state) => {
                let state = state.borrow();
            }
            Msg::FetchMatches => {
                self.matches_store.send(MatchesRequest::Fetch);
            }
            Msg::ReceiveMatchUpdate(Ok(match_item)) => {
                self.matches_store.send(MatchesRequest::Update(match_item));
            }
            Msg::ReceiveMatchUpdate(Err(err)) => {
                log::error!("WebSocket message decode error");
            }
            Msg::UserStore(_) => {}
            Msg::InitUser => {
                self.user_store.send(UserRequest::InitializeUser);
            }
        }
        false
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <BrowserRouter>
                <Switch<MainRoute> render={Switch::render(switch_main)} />
            </BrowserRouter>
        }
    }
}

fn switch_main(routes: &MainRoute) -> Html {
    match routes.clone() {
        MainRoute::Live | MainRoute::Home => {
            gloo::utils::document().set_title("Starbet Live");
            html! {
                <Layout>
                    <LivePage />
                </Layout>
            }
        }
        MainRoute::Upcoming => {
            gloo::utils::document().set_title("Upcoming | Starbet Live");
            html! {
                <Layout>
                    <UpcomingPage />
                </Layout>
            }
        }
        MainRoute::Results => {
            gloo::utils::document().set_title("Results | Starbet Live");
            html! {
                <Layout>
                    <ResultsPage />
                </Layout>
            }
        }
        MainRoute::Registration => {
            gloo::utils::document().set_title("Registration | Starbet Live");
            html! { <RegistrationPage /> }
        }
        MainRoute::About => {
            gloo::utils::document().set_title("About | Starbet Live");
            html! {
                <LayoutNoSidebars>
                    <AboutPage />
                </LayoutNoSidebars>
            }
        }
        MainRoute::PrivacyPolicy => {
            gloo::utils::document().set_title("Privacy policy | Starbet Live");
            html! {
                <LayoutNoSidebars>
                    <PrivacyPolicyPage />
                </LayoutNoSidebars>
            }
        }
        MainRoute::Contact => {
            gloo::utils::document().set_title("Contact | Starbet Live");
            html! {
                <LayoutNoSidebars>
                    <ContactPage />
                </LayoutNoSidebars>
            }
        }
        MainRoute::Profile => {
            gloo::utils::document().set_title("Profile | Starbet Live");
            html! {
                <LayoutNoSidebars>
                    <LayoutProfile>
                        <Switch<ProfileRoute> render={Switch::render(switch_profile)} />
                    </LayoutProfile>
                </LayoutNoSidebars>
            }
        }
        MainRoute::Administration => {
            gloo::utils::document().set_title("Administration | Starbet Live");
            html! {
                <LayoutNoSidebars>
                    <AdministrationPage />
                </LayoutNoSidebars>
            }
        }
        MainRoute::NotFound => {
            gloo::utils::document().set_title("Not found | Starbet Live");
            html! {
                <LayoutNoSidebars>
                    <NotFoundPage />
                </LayoutNoSidebars>
            }
        }
    }
}

fn switch_profile(route: &ProfileRoute) -> Html {
    match route {
        ProfileRoute::Summary => html! { <ProfileSummary /> },
        ProfileRoute::Statistics => html! { <ProfileStatistics /> },
        ProfileRoute::Tickets => html! { <ProfileTickets /> },
        ProfileRoute::NotFound => html! {
            <Redirect<MainRoute> to={MainRoute::NotFound}/>
        },
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
