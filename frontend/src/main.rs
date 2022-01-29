use crate::components::{
    layout::Layout, layout_no_sidebars::LayoutNoSidebars, layout_profile::LayoutProfile,
};
use crate::pages::{
    about_page::AboutPage, contact_page::ContactPage, live_page::LivePage, not_found::NotFoundPage,
    privacy_policy_page::PrivacyPolicyPage, results_page::ResultsPage, upcoming_page::UpcomingPage,
};
use pages::registration_page::RegistrationPage;
use types::{MainRoute, ProfileRoute};
use yew::prelude::*;
use yew_router::{prelude::Redirect, Switch, BrowserRouter};

mod components;
mod pages;
mod services;
mod store;
mod types;

enum Msg {}

struct App {}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
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
        ProfileRoute::Summary => html! {<h1>{"Profile"}</h1>},
        ProfileRoute::Statistics => html! {<h1>{"Statistics"}</h1>},
        ProfileRoute::Tickets => html! {<h1>{"Tickets"}</h1>},
        ProfileRoute::NotFound => html! {
            <Redirect<MainRoute> to={MainRoute::NotFound}/>
        },
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
