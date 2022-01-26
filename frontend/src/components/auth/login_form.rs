use anyhow::anyhow;
use reqwest::{Client, StatusCode};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use std::time::Duration;
use yew::prelude::*;
// use yew::services::{Task, TimeoutService};
use yewtil::future::LinkFuture;

#[derive(PartialEq)]
enum SubmitResult {
    None,
    Success,
    Error,
}

#[derive(Serialize, Deserialize, Clone)]

pub struct LoginForm {}

pub enum Msg {}

impl Component for LoginForm {
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
                <form>

                </form>

            </>
        }
    }
}
