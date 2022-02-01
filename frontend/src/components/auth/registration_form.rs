use super::input::InputType;
use crate::types::users::Field;
use crate::{
    components::{auth::input::TextInput, loading_animation::LoadingAnimation},
    types::{SubmitResult, UserRegistrationFormData},
};
use chrono::{DateTime, Utc};
use gloo_timers::callback::Timeout;
use log::{info, warn};
use yew::prelude::*;

// pub mod user {
//     include!(concat!(env!("OUT_DIR"), concat!("/user.rs")));
// }

// use user::{user_service_client, CreateUserReply, CreateUserRequest, Address};

pub enum Msg {
    Submit,
    SetLoading(bool),
    SetData((String, Field, bool)),
    // ReceiveResponse(Result<CreateMatchReply, Box<dyn std::error::Error>>),
}

pub struct RegistrationForm {
    is_loading: bool,
    error: String,
    submit_result: SubmitResult,
    data: UserRegistrationFormData,
}

impl Component for RegistrationForm {
    type Message = Msg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            is_loading: false,
            error: String::new(),
            submit_result: SubmitResult::None,
            data: UserRegistrationFormData::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Submit => {
                if !self.data.is_valid() {
                    self.error = "Fill all inputs correctly".to_string();
                    return true;
                } else {
                    self.error = String::new();
                }
                ctx.link().send_message(Msg::SetLoading(true));
                // convert date string to chrono datetime and check age
                let datetime = match DateTime::parse_from_rfc3339(&format!(
                    "{}T00:00:00+01:00",
                    self.data.date_of_birth.0
                )) {
                    Ok(val) => val,
                    _ => {
                        warn!("Inserted date is not valid");
                        return false;
                    }
                };
                let datetime_utc = datetime.with_timezone(&Utc);
                // add age check

                // let grpc_client =
                //     user_service_client::UserService::new(String::from("http://127.0.0.1:5430"));

                // ctx.link().send_future(async move {
                //     Msg::ReceiveResponse(
                //         grpc_client
                //             .create_game(CreateUserRequest {  
                //                 first_name: self.data.first_name.0.trim().to_string(),
                //                 last_name: self.data.last_name.0.trim().to_string(),
                //                 civil_id_number: self.data.civil_id_number.0.trim().to_string(),
                //                 date_of_birth: self.data.date_of_birth.0.trim().to_string(),
                //                 email: self.data.email.0.trim().to_string(),
                //                 phone_number: self.data.phone_number.0.trim().to_string(),
                //                 address: self.data.address,
                // })
                //             .await,
                //     )
                // });

                ctx.link().send_message(Msg::SetLoading(false));
            }
            Msg::SetLoading(val) => {
                self.is_loading = val;
            }
            Msg::SetData((new_data, field, is_valid)) => {
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
            // Msg::ReceiveResponse(Ok(_)) => {
            //     self.submit_result = SubmitResult::Success;
            //     let link = ctx.link().clone();
            //     Timeout::new(5000, move || link.send_message(Msg::ResetSubmitResult)).forget();
            // }
            // Msg::ReceiveResponse(Err(_)) => {
            //     self.submit_result = SubmitResult::Error;
            //     let link = ctx.link().clone();
            //     Timeout::new(5000, move || link.send_message(Msg::ResetSubmitResult)).forget();
            // }
            // Msg::ResetSubmitResult => {
            //     self.submit_result = SubmitResult::None;
            // }
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
                            on_change={ctx.link().callback(Msg::SetData)}
                        />
                        <TextInput
                            field={Field::LastName}
                            label="Last name"
                            placeholder="Barrow"
                            on_change={ctx.link().callback(Msg::SetData)}
                        />
                        <TextInput
                            field={Field::CivilIdNumber}
                            label="Civil Id Number"
                            placeholder="XY837923"
                            on_change={ctx.link().callback(Msg::SetData)}
                        />
                        // date input doesn't have proper validation
                        <TextInput
                            input_type={InputType::Date}
                            field={Field::DateOfBirth}
                            label="Date Of Birth"
                            placeholder="hahah"
                            on_change={ctx.link().callback(Msg::SetData)}
                        />
                        <TextInput
                            input_type={InputType::Email}
                            field={Field::Email}
                            label="Email address"
                            placeholder="marcbarrow@email.com"
                            on_change={ctx.link().callback(Msg::SetData)}
                        />
                        <TextInput
                            field={Field::PhoneNumber}
                            label="Phone number"
                            placeholder="+420 913 328 857"
                            on_change={ctx.link().callback(Msg::SetData)}
                        />
                        <TextInput
                            input_type={InputType::Password}
                            field={Field::Password}
                            label="Password"
                            placeholder="******"
                            on_change={ctx.link().callback(Msg::SetData)}
                        />
                        <TextInput
                            input_type={InputType::Password}
                            field={Field::PasswordConfirmation}
                            label="Password confirmation"
                            placeholder="******"
                            on_change={ctx.link().callback(Msg::SetData)}
                        />
                    </div>
                    <div>
                        <TextInput
                            field={Field::StreetName}
                            label="Street name"
                            placeholder="Eagle Drive"
                            on_change={ctx.link().callback(Msg::SetData)}
                        />
                        <TextInput
                            field={Field::StreetNumber}
                            label="Street number"
                            placeholder="1218"
                            on_change={ctx.link().callback(Msg::SetData)}
                        />
                        <TextInput
                            field={Field::City}
                            label="City"
                            placeholder="Southfield"
                            on_change={ctx.link().callback(Msg::SetData)}
                        />
                        <TextInput
                            field={Field::Area}
                            label="Area"
                            placeholder=""
                            on_change={ctx.link().callback(Msg::SetData)}
                            required={false}
                        />
                        <TextInput
                            field={Field::PostalCode}
                            label="Postal code"
                            placeholder="48034"
                            on_change={ctx.link().callback(Msg::SetData)}
                        />
                        <TextInput
                            field={Field::Country}
                            label="Country"
                            placeholder="United States"
                            on_change={ctx.link().callback(Msg::SetData)}
                        />
                        <div>
                            <input type="checkbox" id="conditions" name="conditions" value="conditions" required={true} />
                            <label for="conditions">{" Accept conditions"}</label>
                        </div>
                    </div>
                </div>
                {
                    if self.is_loading {
                        html! { <LoadingAnimation color="dark-blue" /> }
                    } else {
                        html! { }
                    }
                }
                // {
                //     if self.submit_result == SubmitResult::Success {
                //         html! {
                //             <div class="mx-auto my-1 p-1 w-full lg:w-9/12 text-center bg-success-light text-success rounded-md transition-all">
                //                 {"User successfully registered"}
                //             </div>
                //         }
                //     } else if self.submit_result == SubmitResult::Error {
                //         html! {
                //             <div class="mx-auto my-1 p-1 w-full lg:w-9/12 text-center bg-danger-light text-danger rounded-md transition-all">
                //                 {"Something went wrong :( please try again later"}
                //             </div>
                //         }
                //     } else {
                //         html! {}
                //     }
                // }
                {
                    if !self.error.is_empty() {
                        html! {
                            <div class="mx-auto my-1 p-1 w-9/12 lg:w-6/12 text-center bg-danger-light text-danger rounded-md transition-all">
                                {self.error.clone()}
                            </div>
                        }
                    } else {
                        html! {}
                    }
                }
                <button type="submit" class="block shadow-md mx-auto my-1 w-6/12 lg:w-4/12 py-2 px-3 bg-yellow font-bold rounded-md transition-all">
                    {"Register"}
                </button>
            </form>
        }
    }
}
