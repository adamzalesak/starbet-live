use yew::prelude::*;

use super::date_time::DateTime;

pub enum Msg {}

pub struct Header {
    is_logged: bool,
}

impl Component for Header {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { is_logged: false }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
          <header class="bg-dark-blue flex flex-row items-center justify-between text-white p-3">
            <div class="block w-4/12 md:w-2/12 transition-all">
              <img src="assets/icons/starbet-live-yellow.svg" alt="starbet live logo" />
            </div>
            <DateTime />
            <div class="">
              
              <span>{"Login form"}</span>
            </div>
          </header>
        }
    }
}
