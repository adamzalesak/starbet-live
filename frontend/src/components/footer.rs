use yew::prelude::*;
use yew_router::prelude::Link;

use crate::Route;

pub enum Msg {}

pub struct Footer {}

#[derive(Properties, PartialEq)]
pub struct Props {
    #[prop_or(true)]
    pub squared_design: bool,
}

impl Component for Footer {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let Props { squared_design } = ctx.props().clone();

        html! {
            // <div class={"bg-dark-blue text-white text-center text-sm p-2"}>
            <div class={format!("bg-dark-blue text-white text-center text-sm p-2 {}", if *squared_design {"rounded-md py-4"} else {""})}>

                <div class={format!("flex {}", if *squared_design {"flex-col"} 
                                            else {"flex-row flex-wrap justify-evenly w-full sm:w-10/12 lg:w-8/12 mx-auto"})}>
                    
                    <div class={format!("font-medium {}", if *squared_design {"m-2"} else {"m-1"})}>
                        {"© Starbet Live 2022"}
                    </div>
                    <Link<Route> to={Route::About} classes={"underline m-1"}>
                        {"About"}
                    </Link<Route>>
                    <Link<Route> to={Route::PrivacyPolicy} classes={"underline m-1"}>
                        {"Privacy Policy"}
                    </Link<Route>>
                    <Link<Route> to={Route::Contact} classes={"underline m-1"}>
                        {"Contact"}
                    </Link<Route>>
                </div>
            </div>
        }
    }
}
