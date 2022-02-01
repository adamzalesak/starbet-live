use yew::prelude::*;

pub enum Msg {}

pub struct LoadingAnimation {}

#[derive(Properties, PartialEq)]
pub struct LoadingAnimationProps {
    #[prop_or("".to_string())]
    pub color: String,
}

impl Component for LoadingAnimation {
    type Message = Msg;
    type Properties = LoadingAnimationProps;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let LoadingAnimationProps { color } = ctx.props().clone();
        html! {
            <div class="spinner">
                <div class={format!("bounce1 {}", if color.is_empty() {"bg-white".to_string()} else {format!("bg-{}", color)})}></div>
                <div class={format!("bounce2 {}", if color.is_empty() {"bg-white".to_string()} else {format!("bg-{}", color)})}></div>
                <div class={format!("bounce3 {}", if color.is_empty() {"bg-white".to_string()} else {format!("bg-{}", color)})}></div>
            </div>
        }
    }
}
