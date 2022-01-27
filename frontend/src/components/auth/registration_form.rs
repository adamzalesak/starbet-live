use super::input::InputType;
use crate::types::users::Field;
use crate::{components::auth::input::TextInput, types::users::UserRegistrationFormData};
use log::info;
use yew::prelude::*;

pub enum Msg {
    Submit,
    SetData((String, Field)),
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
                info!("Submiting registration form {:?}", self.data);
            }
            Msg::SetData((new_data, field)) => {
                // info!("Arrived to form: {} {:?}", new_data, field);
                match field {
                    Field::FirstName => self.data.first_name = new_data,
                    Field::LastName => self.data.last_name = new_data,
                    Field::Password => self.data.password = new_data,
                    Field::CivilIdNumber => self.data.civil_id_number = new_data,
                    Field::DateOfBirth => self.data.date_of_birth = new_data,
                    Field::Email => self.data.email = new_data,
                    Field::PhoneNumber => self.data.phone_number = new_data,
                    Field::StreetName => self.data.address.street_name = new_data,
                    Field::StreetNumber => self.data.address.street_number = new_data,
                    Field::City => self.data.address.city = new_data,
                    Field::Area => self.data.address.area = Some(new_data),
                    Field::PostalCode => self.data.address.postal_code = new_data,
                    Field::Country => self.data.address.country = new_data,
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
                <div class="w-full h-full lg:w-8/12 py-3 px-5 sm:w-10/12 mx-auto overflow-auto bg-light-grey transition-all ">
                    <div class="text-center font-bold">
                        {"Enter your contact information"}
                    </div>
                    <form onsubmit={ ctx.link().callback(|e: FocusEvent| { e.prevent_default(); Msg::Submit }) }>

                        <TextInput
                            field={Field::FirstName}
                            label="First name"
                            placeholder="kokotia hlava"
                            value={self.data.first_name.clone()}
                            on_change={ctx.link().callback(Msg::SetData)}
                            // is_valid={}
                        />

                        <TextInput
                            field={Field::LastName}
                            label="Last name"
                            placeholder="hahah"
                            value={self.data.last_name.clone()}
                            on_change={ctx.link().callback(Msg::SetData)}
                            // is_valid={is_valid_last_name}
                        />

                        // <TextInput
                        //     field={Field::CivilIdNumber}
                        //     label="Civil Id Number"
                        //     placeholder="hahah"
                        //     value={self.data.civil_id_number.clone()}
                        //     on_change={ctx.link().callback(Msg::SetData)}
                        //     // is_valid={is_valid_last_name}
                        // />

                        // <TextInput
                        //     field={Field::DateOfBirth}
                        //     label="Date Of Birth"
                        //     placeholder="hahah"
                        //     value={self.data.date_of_birth.clone()}
                        //     on_change={ctx.link().callback(Msg::SetData)}
                        //     // is_valid={is_valid_last_name}
                        // />

                        // <TextInput
                        //     input_type={InputType::Email}
                        //     field={Field::Email}
                        //     label="Email"
                        //     placeholder="email"
                        //     value={self.data.email.clone()}
                        //     on_change={ctx.link().callback(Msg::SetData)}
                        //     // is_valid={is_valid_last_name}
                        // />

                        <button type="submit">{"Register"}</button>
                    </form>

                </div>
            </>
        }
    }
}
