use crate::{components::auth::input::TextInput, types::users::UserRegistrationFormData};
use log;
use yew::prelude::*;

pub enum Msg {
    Submit,
    SetName(String),
}

pub struct RegistrationForm {
    data: UserRegistrationFormData,
}

impl Component for RegistrationForm {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            data: UserRegistrationFormData::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Submit => {
                log::info!(
                    "Submiting registration form: input {:?}",
                    &self.data.first_name
                );
            }
            Msg::SetName(new_name) => {
                self.data.address.country = new_name;
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let on_change_name = ctx.link().callback(Msg::SetName);
        let on_change_password = ctx.link().callback(Msg::SetName);
        html! {
            <>
                <div class="w-full lg:w-8/12 p-2 sm:w-10/12 overflow-auto mx-auto transition-all">
                    {"RegistrationPage"}
                    <form onsubmit={ ctx.link().callback(|e: FocusEvent| { e.prevent_default(); Msg::Submit }) }>

                        <TextInput
                            label={"First name:".to_string()}
                            placeholder={"kokotia hlava".to_string()}
                            value={self.data.first_name.clone()}
                            on_change={on_change_name}
                        />

                        <TextInput
                            input_type={"password"}
                            label={"First name:".to_string()}
                            value={self.data.password.clone()}
                            on_change={on_change_password}
                        />

                        <button type="submit">{"Register"}</button>
                    </form>

                </div>
            </>
        }
    }
}
