use crate::components::{layout::Layout, layout_no_sidebars::LayoutNoSidebars};
use crate::pages::{
    about_page::AboutPage, contact_page::ContactPage, live_page::LivePage, not_found::NotFoundPage,
    privacy_policy_page::PrivacyPolicyPage, profile_page::ProfilePage, results_page::ResultsPage,
    upcoming_page::UpcomingPage,
};
use pages::registration_page::RegistrationPage;
use yew::prelude::*;
use yew_router::prelude::*;

mod agents;
mod components;
mod pages;
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
            html! {
                <Layout>
                    <LivePage />
                </Layout>
            }
        }
        Route::Upcoming => {
            html! {
                <Layout>
                    <UpcomingPage />
                </Layout>
            }
        }
        Route::Results => {
            html! {
                <Layout>
                    <ResultsPage />
                </Layout>
            }
        }
        Route::Registration => html! { <RegistrationPage /> },
        Route::About => {
            html! {
                <LayoutNoSidebars>
                    <AboutPage />
                </LayoutNoSidebars>
            }
        }
        Route::PrivacyPolicy => {
            html! {
                <LayoutNoSidebars>
                    <PrivacyPolicyPage />
                </LayoutNoSidebars>
            }
        }
        Route::Contact => {
            html! {
                <LayoutNoSidebars>
                    <ContactPage />
                </LayoutNoSidebars>
            }
        }
        Route::Profile => {
            html! {
                <LayoutNoSidebars>
                    <ProfilePage />
                </LayoutNoSidebars>
            }
        }
        Route::NotFound => {
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
