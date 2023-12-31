use crate::types::MainRoute;
use yew::prelude::*;
use yew_router::prelude::Link;

pub enum Msg {}

pub struct Footer {}

#[derive(Properties, PartialEq)]
pub struct FooterProps {
    #[prop_or(true)]
    pub squared_design: bool,
}

impl Component for Footer {
    type Message = Msg;
    type Properties = FooterProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let FooterProps { squared_design } = *ctx.props();

        html! {
            // <div class={"bg-dark-blue text-white text-center text-sm p-2"}>
            <div class={format!("bg-dark-blue text-white text-center text-sm p-2 {}", 
                if squared_design {"rounded-md py-4 h-1/5 flex flex-col justify-center"} else {""})}>

                <div class={format!("flex {}", if squared_design {"flex-col"}
                                            else {"flex-row flex-wrap justify-evenly w-full sm:w-10/12 lg:w-8/12 mx-auto"})}>

                    <div class={format!("font-medium {}", if squared_design {"m-2"} else {"m-1"})}>
                        {"© Starbet Live 2022"}
                    </div>
                    <Link<MainRoute> to={MainRoute::About} classes={"underline m-1 transition-all footer-link"}>
                        {"About"}
                    </Link<MainRoute>>
                    <Link<MainRoute> to={MainRoute::PrivacyPolicy} classes={"underline m-1 transition-all footer-link"}>
                        {"Privacy Policy"}
                    </Link<MainRoute>>
                    <Link<MainRoute> to={MainRoute::Contact} classes={"underline m-1 transition-all footer-link"}>
                        {"Contact"}
                    </Link<MainRoute>>
                </div>
            </div>
        }
    }
}
