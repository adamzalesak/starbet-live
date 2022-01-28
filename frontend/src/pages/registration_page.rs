use crate::{
    components::{
        auth::registration_form::RegistrationForm, footer::Footer, header::date_time::DateTime,
    },
    Route,
};
use yew::prelude::*;
use yew_router::prelude::Link;

pub enum Msg {}

pub struct RegistrationPage {}

impl Component for RegistrationPage {
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
            <>
                <header class="bg-dark-blue flex flex-row justify-between text-white">
                    <div class="block w-52 p-2 mx-auto text-center">
                        <Link<Route> to={Route::Home}>
                            <img src="/starbet-live-yellow.svg" alt="starbet live logo" class="mb-1"/>
                        </Link<Route>>
                        <DateTime />
                    </div>
                </header>
                <div class="flex flex-col h-full overflow-auto">
                    <div class="w-full lg:w-8/12 py-3 px-5 sm:w-10/12 mx-auto bg-light-grey transition-all main-grow">
                        <div class="text-center font-bold">
                            {"Enter your contact information"}
                        </div>
                        <RegistrationForm />
                    </div>
                    <Footer squared_design={false} />
                </div>
            </>
        }
    }
}
