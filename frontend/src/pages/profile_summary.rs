use crate::{store::UserStore, types::grpc_types::user::User};
use log::{error, info};
use yew::prelude::*;
use yew_agent::{
    utils::store::{Bridgeable, ReadOnly, StoreWrapper},
    Bridge,
};

pub enum Msg {
    SetLoading(bool),
    UserStore(ReadOnly<UserStore>),
}

pub struct ProfileSummary {
    is_loading: bool,
    user: Option<User>,
    user_store: Box<dyn Bridge<StoreWrapper<UserStore>>>,
}

impl Component for ProfileSummary {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            user: None,
            user_store: UserStore::bridge(ctx.link().callback(Msg::UserStore)),
            is_loading: false,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::SetLoading(val) => {
                self.is_loading = val;
                true
            }
            Msg::UserStore(state) => {
                let state = state.borrow();
                self.user = match &state.user {
                    Some(val) => Some(val.clone()),
                    None => None,
                };
                true
            }
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        let (
            first_name,
            last_name,
            civil_id_number,
            date_of_birth,
            email,
            phone_number,
            (street_name, street_number, city, area, postal_code, country),
        ) = match &self.user {
            Some(val) => (
                val.first_name.clone(),
                val.last_name.clone(),
                val.civil_id_number.clone(),
                val.date_of_birth.clone(),
                val.email.clone(),
                val.phone_number.clone(),
                match &val.address {
                    Some(adr) => (
                        adr.street_name.clone(),
                        adr.street_number.clone(),
                        adr.city.clone(),
                        match &adr.area {
                            Some(x) => x.clone(),
                            None => String::new(),
                        },
                        adr.postal_code.clone(),
                        adr.country.clone(),
                    ),
                    None => (
                        String::new(),
                        String::new(),
                        String::new(),
                        String::new(),
                        String::new(),
                        String::new(),
                    ),
                },
            ),
            None => (
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                String::new(),
                (
                    String::new(),
                    String::new(),
                    String::new(),
                    String::new(),
                    String::new(),
                    String::new(),
                ),
            ),
        };
        html! {
            <div class="lg:grid lg:grid-cols-2 bg-light-grey">
                <div class="px-14 py-2">
                    <div class="mt-2 mb-4 font-bold text-lg">{"Personal information"}</div>
                    <div class="my-2">
                        <div class="border-b border-black">{"First name:"}</div>
                        <div class="font-bold">{first_name}</div>
                    </div>
                    <div class="my-2">
                        <div class="border-b border-black">{"Last name:"}</div>
                        <div class="font-bold">{last_name}</div>
                    </div>
                    <div class="my-2">
                        <div class="border-b border-black">{"Civil ID number:"}</div>
                        <div class="font-bold">{civil_id_number}</div>
                    </div>
                    <div class="my-2">
                        <div class="border-b border-black">{"Birth date:"}</div>
                        <div class="font-bold">{date_of_birth}</div>
                    </div>
                    <div class="my-2">
                        <div class="border-b border-black">{"Email:"}</div>
                        <div class="font-bold">{email}</div>
                    </div>
                    <div class="my-2">
                        <div class="border-b border-black">{"Phone number:"}</div>
                        <div class="font-bold">{phone_number}</div>
                    </div>
                </div>
                <div class="px-14 py-2">
                    <div class="mt-2 mb-4 font-bold text-lg">{"Address"}</div>
                    <div class="my-2">
                        <div class="border-b border-black">{"Street name:"}</div>
                        <div class="font-bold">{street_name}</div>
                    </div>
                    <div class="my-2">
                        <div class="border-b border-black">{"Street number:"}</div>
                        <div class="font-bold">{street_number}</div>
                    </div>
                    <div class="my-2">
                        <div class="border-b border-black">{"City:"}</div>
                        <div class="font-bold">{city}</div>
                    </div>
                    <div class="my-2">
                        <div class="border-b border-black">{"Area:"}</div>
                        <div class="font-bold">{area}</div>
                    </div>
                    <div class="my-2">
                        <div class="border-b border-black">{"Postal code:"}</div>
                        <div class="font-bold">{postal_code}</div>
                    </div>
                    <div class="my-2">
                        <div class="border-b border-black">{"Country:"}</div>
                        <div class="font-bold">{country}</div>
                    </div>
                </div>
            </div>
        }
    }
}
