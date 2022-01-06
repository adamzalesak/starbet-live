use yew::prelude::*;

use crate::components::header::header::Header;
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
                { ctx.props().children.clone() }
            </>
        }
    }
}
