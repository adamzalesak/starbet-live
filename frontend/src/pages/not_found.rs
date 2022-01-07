use yew::prelude::*;

pub enum Msg {}

pub struct NotFoundPage {}

impl Component for NotFoundPage {
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
         <div class="h-max flex flex-col items-center justify-center mt-8 font-bold">
            <span class="text-9xl">{"404"}</span>
            <span>{"Whoops, page not found"}</span>
        </div>
        }
    }
}
