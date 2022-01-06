use gloo::timers::callback::Interval;
use wasm_bindgen::JsValue;
use yew::{html, Component, Context, Html};

pub enum Msg {
    UpdateTime,
    Updatedate,
}

pub struct DateTime {
    time: String,
    date: String,
    _standalone: (Interval, Interval),
}

impl DateTime {
    fn get_current_time() -> String {
        let date = js_sys::Date::new_0();
        String::from(date.to_locale_time_string("sk-SK"))
    }
    fn get_current_date() -> String {
        let date = js_sys::Date::new_0();
        String::from(date.to_locale_date_string("sk-SK", &JsValue::from_str("{}")))
    }
}

impl Component for DateTime {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        let clock_handle = {
            let link = ctx.link().clone();
            Interval::new(500, move || link.send_message(Msg::UpdateTime))
        };

        let date_handle = {
            let link = ctx.link().clone();
            // update date every 2 minutes
            Interval::new(1000 * 60 * 2, move || link.send_message(Msg::Updatedate))
        };

        Self {
            time: DateTime::get_current_time(),
            date: DateTime::get_current_date(),
            _standalone: (clock_handle, date_handle),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateTime => {
                self.time = DateTime::get_current_time();
                true
            }
            Msg::Updatedate => {
                self.time = DateTime::get_current_date();
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div>
                { &self.date } 
                {" "}
                { &self.time }
            </div>
        }
    }
}