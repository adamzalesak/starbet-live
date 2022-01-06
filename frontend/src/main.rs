use yew::prelude::*;
use yew_router::prelude::*;

mod components;
mod pages;
use crate::components::{
    layout::Layout,
    main::{live::Live, results::Results, upcoming::Upcoming},
};
use crate::pages::not_found::NotFoundPage;

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
            html! { <Live /> }
        }
        Route::Upcoming => {
            html! { <Upcoming /> }
        }
        Route::Results => {
            html! { <Results /> }
        }
        Route::NotFound => {
            html! { <NotFoundPage /> }
        }
        Route::Home => {
            html! { <Live /> }
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::start_app::<App>();
}
