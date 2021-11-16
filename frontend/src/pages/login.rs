use std::rc::Rc;

use yew::prelude::*;
use yew_router::replace_route;
use yewdux_functional::use_store;

use crate::{
    components::{login::LoginBox, GuestNavBar},
    state::{AppState, AppStateStore, GetState, LoginErrorState, LoginErrorStateStore},
    Route,
};

#[function_component(Login)]
pub fn login() -> Html {
    let store = use_store::<AppStateStore>();
    let app_state = store.get_state();
    let err_store = use_store::<LoginErrorStateStore>();
    let err_state = err_store.get_state();

    match app_state.as_ref() {
        AppState::Guest | AppState::RequestedGuest(_) => {
            html! {<LoginGuest err={err_state}/>}
        }
        AppState::Tutor(_) | AppState::Admin(_) => {
            replace_route(Route::Index);
            html! {}
        }
    }
}

#[derive(Properties, PartialEq)]
struct LoginGuestProps {
    err: Rc<LoginErrorState>,
}

#[function_component(LoginGuest)]
fn login_guest(props: &LoginGuestProps) -> Html {
    let err_message = match &props.err.as_ref().0 {
        Some(err) => html! {
            <div class="notification is-danger">
                <p>{err}</p>
            </div>
        },
        None => html! {},
    };

    html! {
        <section class="hero is-info is-fullheight">
            <div class="hero-head">
                <GuestNavBar/>
            </div>

            <div class="hero-body section">
                <div class="container">
                    <LoginBox/>
                    {err_message}
                </div>
            </div>
        </section>
    }
}
