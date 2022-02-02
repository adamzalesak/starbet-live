use crate::components::administration::{
    change_match_event::ChangeMatchEvent, create_game_form::CreateGameForm,
    create_match_form::CreateMatchForm, create_team_form::CreateTeamForm,
    create_team_plays_game_form::CreateTeamPlaysGameForm,
};
use crate::store::UserStore;
use crate::types::{MainRoute, UserInfo};
use yew::prelude::*;
use yew_agent::{
    utils::store::{Bridgeable, ReadOnly, StoreWrapper},
    Bridge,
};
use yew_router::prelude::*;

pub enum Msg {
    UserStore(ReadOnly<UserStore>),
}

pub struct AdministrationPage {
    user: UserInfo,
    _user_store: Box<dyn Bridge<StoreWrapper<UserStore>>>,
}

impl Component for AdministrationPage {
    type Message = Msg;
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            user: UserInfo::new(),
            _user_store: UserStore::bridge(ctx.link().callback(Msg::UserStore)),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UserStore(state) => {
                let state = state.borrow();
                self.user = state.user.clone();

                // only authenticated user with admin rights can access
                if !state.user.is_authenticated() || state.user.id != 0 {
                    let history = ctx.link().history().unwrap();
                    history.push(MainRoute::Home);
                }
            }
        }
        true
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        html! {
            <div class="admin-page">
                <CreateGameForm />
                <CreateTeamForm />
                <CreateTeamPlaysGameForm />
                <CreateMatchForm />
                <ChangeMatchEvent />
            </div>
        }
    }
}
