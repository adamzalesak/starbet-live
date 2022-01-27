use yew::prelude::*;

pub enum Msg {}

pub struct MatchItem {}

impl Component for MatchItem {
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
            <div class="px-2 bg-white rounded-md flex flex-col lg:flex-row lg:gap-2 pb-2 relative">
                <div class="w-1/3">
                    <div class="font-bold absolute">
                        <span>{"Fnatic"}</span>
                        {" "}
                        <span class="text-yellow">{"13 kills - 17 kills"}</span>
                        {" "}
                        <span>{"SKT T1"}</span>
                    </div>
                    <div class="mt-5 w-full">{"Time -> 25:31"}</div>
                </div>
                <div class="w-full lg:w-2/3">
                    <div class="text-center">{"Winner"}</div>
                    <div class="flex gap-5">
                        <button class="bg-light-grey rounded-md w-full p-1 px-3 flex gap-1 justify-between cursor-pointer">
                            <span class="font-bold">{"Fnatic"}</span>
                            <span>{"2.15"}</span>
                        </button>
                        <button class="bg-light-grey rounded-md w-full p-1 px-3 flex gap-1 justify-between cursor-pointer">
                            <span class="font-bold">{"SKT T1"}</span>
                            <span>{"1.63"}</span>
                        </button>
                    </div>
                </div>
            </div>
        }
    }
}
