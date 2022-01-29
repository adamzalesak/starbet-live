use crate::{
    components::auth::input::{InputType, TextInput},
    types::{Field, MainRoute, UserLoginFormData},
};
use log::info;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_router::prelude::Link;

#[derive(Serialize, Deserialize, Clone)]

pub struct LoginForm {
    data: UserLoginFormData,
}

pub enum Msg {
    Submit,
    SetEmail((String, Field, bool)),
    SetPassword((String, Field, bool)),
}

impl Component for LoginForm {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            data: UserLoginFormData::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Submit => {
                if self.data.email.is_empty() || self.data.password.is_empty() {
                    return false;
                }

                // request

                // response

                info!("Submiting login form");
            }
            Msg::SetEmail((new_data, _, _)) => self.data.email = new_data,
            Msg::SetPassword((new_data, _, _)) => self.data.password = new_data,
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <form onsubmit={ ctx.link().callback(|e: FocusEvent| { e.prevent_default(); Msg::Submit }) }
                        class="flex flex-row gap-1 text-black login" >
                    <div class="flex flex-col gap-1 lg:flex-row">
                        <TextInput
                            input_type={InputType::Email}
                            field={Field::Email}
                            label="Email address"
                            placeholder="EMAIL"
                            on_change={ctx.link().callback(Msg::SetEmail)}
                            required={false}
                        />
                        <TextInput
                            input_type={InputType::Password}
                            field={Field::Password}
                            label="Password"
                            placeholder="PASSWORD"
                            on_change={ctx.link().callback(Msg::SetPassword)}
                            required={false}
                        />
                    </div>
                    <div class="flex flex-row gap-1">
                        <button type="submit"
                                class="block my-auto px-1 py-4 lg:py-1 bg-light-grey uppercase font-light rounded-md transition-all">
                            {"Login"}
                        </button>
                        <Link<MainRoute> to={MainRoute::Registration}
                                classes="block my-auto px-1 py-4 lg:py-1 bg-yellow rounded-md uppercase font-light text-black transition-all">
                            { "Join" }
                        </Link<MainRoute>>
                    </div>
                </form>

            </>
        }
    }
}
