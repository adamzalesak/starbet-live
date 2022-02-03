use yew::prelude::*;

pub enum Msg {}

pub struct PrivacyPolicyPage {}

impl Component for PrivacyPolicyPage {
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
                <div class="h-56 bg-danger-light">{"Privacy policy page"}</div>
                <div class="h-56 bg-danger-light">{"Privacy policy page"}</div>
                <div class="h-56 bg-danger-light">{"Privacy policy page"}</div>
                <div class="h-56 bg-danger-light">{"Privacy policy page"}</div>
            </>
        }
    }
}
