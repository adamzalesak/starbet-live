use yew::prelude::*;

use crate::components::header::header::Header;
pub enum Msg {}

pub struct Layout {}

impl Component for Layout {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <Header />
            </>
        }
    }
}
