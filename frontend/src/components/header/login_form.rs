// use anyhow::anyhow;
// use reqwest::{Client, StatusCode};
// use serde::{Deserialize, Serialize};
// use std::sync::Arc;
// use std::time::Duration;
// use yew::prelude::*;
// use yew::services::{Task, TimeoutService};
// use yewtil::future::LinkFuture;

// #[derive(PartialEq)]
// enum SubmitResult {
//     None,
//     Success,
//     Error,
// }

// #[derive(Serialize, Deserialize, Clone)]
// pub struct LoginFormData {
//     email: String,
//     pwd: String,
// }


// pub struct LoginForm {
//     client: Arc<Client>,
//     data: LoginFormData,
//     submit_result: SubmitResult,
//     // https://github.com/yewstack/yew/blob/v0.18/examples/timer/src/main.rs
//     timeout_job: Option<Box<dyn Task>>,
// }

// impl LoginForm {
//     fn new() -> Self {
//         Self {
//             email: String::new(),
//             pwd: String::new(),
//         }
//     }
// }

// pub enum Msg {
//     EmailChange(InputData),
//     PasswordChange(InputData),
//     Submit,
//     SubmitSuccess,
//     SubmitFailure,
//     ResetSubmitResult,
// }

// impl Component for LoginForm {
//     type Message = Msg;
//     type Properties = ();

//     fn create(_ctx: &Context<Self>) -> Self {
//         Self {
//             client: Arc::new(Client::new()),
//             data: LoginFormData::new(),
//             submit_result: SubmitResult::None,
//             timeout_job: None,
//         }
//     }

//     fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
//         false
//     }

//     fn view(&self, ctx: &Context<Self>) -> Html {
//         html! {
//             <form onsubmit={self.link.callback(|e: FocusEvent| { e.prevent_default(); Msg::Submit })} class="flex flex-col items-start my-5 mx-3">

//                 <label for="email" class="">{ "Email" }</label>
//                 <input type="text" class=""
//                     id="email"
//                     value={self.data.email.clone()}
//                     // oninput={self.link.callback(|input_data: InputData| Msg::EmailChange(input_data))}
//                     placeholder="example@mail.com" />

//                 <label for="last-name" class="">{ "Password" }</label>
//                 <input type="text" class=""
//                     id="password"
//                     value={self.data.last_name.clone()}
//                     // oninput={self.link.callback(|input_data: InputData| Msg::PasswordChange(input_data))}
//                     placeholder="*******" />

//                 <button
//                     type="submit"
//                     class="">
//                     { "Login" }
//                 </button>

//             </form>
//             // {
//                 // if self.submit_result == SubmitResult::Success {
//                 //     html! {
//                 //         <div class="flex justify-center mt-3 duration-300">
//                 //             <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
//                 //                 <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M9 12l2 2 4-4m6 2a9 9 0 11-18 0 9 9 0 0118 0z" />
//                 //             </svg>
//                 //             <span class="ml-1">{ "Patient created!" }</span>
//                 //         </div>
//                 //     }
//                 // } else if self.submit_result == SubmitResult::Error {
//                 //     html! {
//                 //         <div class="flex justify-center mt-3 duration-300">
//                 //             <svg xmlns="http://www.w3.org/2000/svg" class="h-6 w-6" fill="none" viewBox="0 0 24 24" stroke="currentColor">
//                 //                 <path stroke-linecap="round" stroke-linejoin="round" stroke-width="1.5" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z" />
//                 //             </svg>
//                 //             <span class="ml-1">{ "Something happened! Try again later." }</span>
//                 //         </div>
//                 //     }
//                 // } else {
//                 //     html! {}
//                 // }
//             // }
//         }
//     }
// }
