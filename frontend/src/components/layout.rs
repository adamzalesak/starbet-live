use yew::prelude::*;

use crate::components::{
    header::header::Header,
    sidebars::{
        footer::Footer,
        games::Games,
        tickets::{latest_tickets::LatestTickets, ticket::Ticket},
    },
};
pub enum Msg {}

pub struct Layout {}

#[derive(Properties, PartialEq)]
pub struct LayoutProps {
    #[prop_or_default]
    pub children: Children,
}

impl Component for Layout {
    type Message = Msg;
    type Properties = LayoutProps;

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
                <main class="w-full flex-auto flex flex-row overflow-auto">
                    <section class="w-3/12 lg:w-2/12 bg-light-grey p-2 flex flex-col justify-between gap-2">
                        <Games />
                        <Footer />
                    </section>
                    <section class="w-6/12 lg:w-8/12 p-2 overflow-auto">
                        { ctx.props().children.clone() }
                    </section>
                    <section class="w-3/12 lg:w-2/12 bg-light-grey p-2 flex flex-col justify-between gap-2">
                        <Ticket />
                        <LatestTickets />
                    </section>
                </main>
            </>
        }
    }
}
