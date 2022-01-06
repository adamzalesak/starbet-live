use yew::prelude::*;

mod components;
use crate::components::layout::Layout;
enum Msg {
}

struct App {
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {  }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <Layout />
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
