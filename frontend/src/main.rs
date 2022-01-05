use yew::prelude::*;

enum Msg {
    AddOne,
}

struct Model {
    value: i64,
}

impl Component for Model {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self { value: 0 }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::AddOne => {
                self.value += 1;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let link = ctx.link();
        html! {
            <div class="bg-sky-300 p-3">
                <button class="pd-2 bg-indigo-600 text-white text-sm leading-6 font-medium py-2 px-3 rounded-lg" onclick={link.callback(|_| Msg::AddOne)}>
                    { self.value }
                </button>
            </div>
        }
    }
}

fn main() {
    yew::start_app::<Model>();
}
