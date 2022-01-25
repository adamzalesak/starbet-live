use crate::components::layout::Layout;
use crate::pages::{
    live_page::LivePage, not_found::NotFoundPage, results_page::ResultsPage,
    upcoming_page::UpcomingPage,
};
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
                <Layout>
                    <Switch<Route> render={Switch::render(switch)} />
                </Layout>
            </BrowserRouter>
        }
    }
}

fn switch(routes: &Route) -> Html {
    match routes.clone() {
        Route::Live => {
            html! { <LivePage /> }
        }
        Route::Upcoming => {
            html! { <UpcomingPage /> }
        }
        Route::Results => {
            html! { <ResultsPage /> }
        }
        Route::NotFound => {
            html! { <NotFoundPage /> }
        }
        Route::Home => {
            html! { <LivePage /> }
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
