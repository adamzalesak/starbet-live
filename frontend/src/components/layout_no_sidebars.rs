use yew::prelude::*;

use crate::components::{footer::Footer, header::header::Header};
pub enum Msg {}

pub struct LayoutNoSidebars {}

#[derive(Properties, PartialEq)]
pub struct LayoutNoSidebarsProps {
    #[prop_or_default]
    pub children: Children,
}

impl Component for LayoutNoSidebars {
    type Message = Msg;
    type Properties = LayoutNoSidebarsProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <Header />
                <div class="flex flex-col h-full overflow-auto">
                    <main class="w-full lg:w-8/12 p-2 sm:w-10/12 mx-auto transition-all main-grow">
                        { ctx.props().children.clone() }
                    </main>
                    <Footer squared_design={false} />
                </div>
            </>
        }
    }
}
