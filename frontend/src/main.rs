use crate::components::{layout::Layout, layout_no_sidebars::LayoutNoSidebars};
use crate::pages::{
    about_page::AboutPage, contact_page::ContactPage, live_page::LivePage, not_found::NotFoundPage,
    privacy_policy_page::PrivacyPolicyPage, profile_page::ProfilePage, results_page::ResultsPage,
    upcoming_page::UpcomingPage,
};
use pages::registration_page::RegistrationPage;
use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;
mod services;
mod store;
mod types;

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/live")]
    Live,
    #[at("/upcoming")]
    Upcoming,
    #[at("/results")]
    Results,
    #[at("/registration")]
    Registration,
    #[at("/about")]
    About,
    #[at("/privacy-policy")]
    PrivacyPolicy,
    #[at("/contact")]
    Contact,
    #[at("/profile")]
    Profile,
    #[at("/")]
    Home,
    #[not_found]
    #[at("/404")]
    NotFound,
}

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
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        }
    }
}

fn switch(routes: &Route) -> Html {
    match routes.clone() {
        Route::Live | Route::Home => {
            gloo::utils::document().set_title("Starbet Live");
            html! {
                <Layout>
                    <LivePage />
                </Layout>
            }
        }
        Route::Upcoming => {
            gloo::utils::document().set_title("Upcoming | Starbet Live");
            html! {
                <Layout>
                    <UpcomingPage />
                </Layout>
            }
        }
        Route::Results => {
            gloo::utils::document().set_title("Results | Starbet Live");
            html! {
                <Layout>
                    <ResultsPage />
                </Layout>
            }
        }
        Route::Registration => {
            gloo::utils::document().set_title("Registration | Starbet Live");
            html! { <RegistrationPage /> }
        }
        Route::About => {
            gloo::utils::document().set_title("About | Starbet Live");
            html! {
                <LayoutNoSidebars>
                    <AboutPage />
                </LayoutNoSidebars>
            }
        }
        Route::PrivacyPolicy => {
            gloo::utils::document().set_title("Privacy policy | Starbet Live");
            html! {
                <LayoutNoSidebars>
                    <PrivacyPolicyPage />
                </LayoutNoSidebars>
            }
        }
        Route::Contact => {
            gloo::utils::document().set_title("Contact | Starbet Live");
            html! {
                <LayoutNoSidebars>
                    <ContactPage />
                </LayoutNoSidebars>
            }
        }
        Route::Profile => {
            gloo::utils::document().set_title("Profile | Starbet Live");
            html! {
                <LayoutNoSidebars>
                    <ProfilePage />
                </LayoutNoSidebars>
            }
        }
        Route::NotFound => {
            gloo::utils::document().set_title("Not found | Starbet Live");
            html! {
                <LayoutNoSidebars>
                    <NotFoundPage />
                </LayoutNoSidebars>
            }
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
