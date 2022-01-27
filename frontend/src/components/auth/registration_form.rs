use super::input::InputType;
use crate::types::users::Field;
use crate::{components::auth::input::TextInput, types::users::UserRegistrationFormData};
use log::info;
use yew::prelude::*;

pub enum Msg {
    Submit,
    SetData((String, Field, bool)),
}

pub struct RegistrationForm {
    error: String,
    data: UserRegistrationFormData,
}

impl Component for RegistrationForm {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            error: String::new(),
            data: UserRegistrationFormData::new(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Submit => {
                if !self.data.is_valid() {
                    self.error = "Fill all inputs correctly".to_string();
                    return true;
                } else {
                    self.error = String::new();
                }
                info!("Submiting registration form {:?}", self.data);
            }
            Msg::SetData((new_data, field, is_valid)) => {
                // info!("Arrived from input: {} {:?} {}", new_data, field, is_valid);
                match field {
                    Field::FirstName => self.data.first_name = (new_data, is_valid),
                    Field::LastName => self.data.last_name = (new_data, is_valid),
                    Field::Password => self.data.password = (new_data, is_valid),
                    Field::PasswordConfirmation => {
                        self.data.password_confirmation = (new_data, is_valid)
                    }
                    Field::CivilIdNumber => self.data.civil_id_number = (new_data, is_valid),
                    Field::DateOfBirth => self.data.date_of_birth = (new_data, is_valid),
                    Field::Email => self.data.email = (new_data, is_valid),
                    Field::PhoneNumber => self.data.phone_number = (new_data, is_valid),
                    Field::StreetName => self.data.address.street_name = (new_data, is_valid),
                    Field::StreetNumber => self.data.address.street_number = (new_data, is_valid),
                    Field::City => self.data.address.city = (new_data, is_valid),
                    Field::Area => self.data.address.area = (Some(new_data), is_valid),
                    Field::PostalCode => self.data.address.postal_code = (new_data, is_valid),
                    Field::Country => self.data.address.country = (new_data, is_valid),
                }
                if !self.data.password.0.is_empty()
                    && !self.data.password_confirmation.0.is_empty()
                    && (self.data.password.0 != self.data.password_confirmation.0
                        && self.data.password.1
                        && self.data.password_confirmation.1)
                {
                    self.error = "Passwords do not match".to_string()
                } else {
                    self.error = String::new();
                }
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <form onsubmit={ ctx.link().callback(|e: FocusEvent| { e.prevent_default(); Msg::Submit }) } >
                <div class="grid md:grid-cols-2 gap-4">
                    <div>
                        <TextInput
                            field={Field::FirstName}
                            label="First name"
                            placeholder="Marc"
                            // value={self.data.first_name.clone()}
                            on_change={ctx.link().callback(Msg::SetData)}
                        />
                        <TextInput
                            field={Field::LastName}
                            label="Last name"
                            placeholder="Barrow"
                            // value={self.data.last_name.clone()}
                            on_change={ctx.link().callback(Msg::SetData)}
                        />
                        <TextInput
                            field={Field::CivilIdNumber}
                            label="Civil Id Number"
                            placeholder="XY837923"
                            // value={self.data.civil_id_number.clone()}
                            on_change={ctx.link().callback(Msg::SetData)}
                        />
                        // date input doesn't have proper validation
                        <TextInput
                            input_type={InputType::Date}
                            field={Field::DateOfBirth}
                            label="Date Of Birth"
                            placeholder="hahah"
                            // value={self.data.date_of_birth.clone()}
                            on_change={ctx.link().callback(Msg::SetData)}
                        />
                        <TextInput
                            input_type={InputType::Email}
                            field={Field::Email}
                            label="Email address"
                            placeholder="marcbarrow@email.com"
                            // value={self.data.email.clone()}
                            on_change={ctx.link().callback(Msg::SetData)}
                        />
                        <TextInput
                            field={Field::PhoneNumber}
                            label="Phone number"
                            placeholder="+420 913 328 857"
                            // value={self.data.phone_number.clone()}
                            on_change={ctx.link().callback(Msg::SetData)}
                        />
                        <TextInput
                            input_type={InputType::Password}
                            field={Field::Password}
                            label="Password"
                            placeholder="******"
                            // value={self.data.phone_number.clone()}
                            on_change={ctx.link().callback(Msg::SetData)}
                        />
                        <TextInput
                            input_type={InputType::Password}
                            field={Field::PasswordConfirmation}
                            label="Password confirmation"
                            placeholder="******"
                            // value={self.data.phone_number.clone()}
                            on_change={ctx.link().callback(Msg::SetData)}
                        />
                    </div>
                    <div>
                        <TextInput
                            field={Field::StreetName}
                            label="Street name"
                            placeholder="Eagle Drive"
                            // value={self.data.street_name.clone()}
                            on_change={ctx.link().callback(Msg::SetData)}
                        />
                        <TextInput
                            field={Field::StreetNumber}
                            label="Street number"
                            placeholder="1218"
                            // value={self.data.street_number.clone()}
                            on_change={ctx.link().callback(Msg::SetData)}
                        />
                        <TextInput
                            field={Field::City}
                            label="City"
                            placeholder="Southfield"
                            // value={self.data.city.clone()}
                            on_change={ctx.link().callback(Msg::SetData)}
                        />
                        <TextInput
                            field={Field::Area}
                            label="Area"
                            placeholder=""
                            // value={self.data.area.clone()}
                            on_change={ctx.link().callback(Msg::SetData)}
                            required={false}
                        />
                        <TextInput
                            field={Field::PostalCode}
                            label="Postal code"
                            placeholder="48034"
                            // value={self.data.postal_code.clone()}
                            on_change={ctx.link().callback(Msg::SetData)}
                        />
                        <TextInput
                            field={Field::Country}
                            label="Country"
                            placeholder="United States"
                            // value={self.data.country.clone()}
                            on_change={ctx.link().callback(Msg::SetData)}
                        />
                        <div class="">
                            <input type="checkbox" id="conditions" name="conditions" value="conditions" required={true} />
                            <label for="conditions">{" Accept conditions"}</label>
                        </div>
                    </div>
                </div>
                {
                    if !self.error.is_empty() {
                        html! {
                            <div class="text-danger text-center mx-auto my-1">
                                <div class="inline-block bg-danger-light py-2 px-3 rounded-md">{self.error.clone()}</div>
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }
                <button type="submit" class="block mx-auto my-1 w-6/12 lg:w-4/12 py-2 px-3 bg-yellow font-bold rounded-md transition-all">
                    {"Register"}
                </button>
            </form>
        }
    }
}
