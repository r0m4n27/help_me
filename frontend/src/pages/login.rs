use yew::prelude::*;
use yewdux_functional::use_store;

use crate::{
    components::{
        login::{LoginBox, LoginErrorStore},
        GuestNavBar,
    },
    state::{AppState, AppStateStore},
};

#[function_component(Login)]
pub fn login() -> Html {
    let store = use_store::<AppStateStore>();
    let app_state = store.state().map(|s| s.as_ref()).unwrap_or_default();

    match app_state {
        AppState::Guest(_) | AppState::RequestedGuest(_, _) => {
            let error_store = use_store::<LoginErrorStore>();
            let login_err = error_store
                .state()
                .map(|e| e.as_ref())
                .and_then(|e| e.clone());

            html! {<LoginGuest err={login_err.map(|e|e.0)}/>}
        }
        AppState::Tutor(_) => html! {},
        AppState::Admin(_) => html! {},
    }
}

#[derive(Properties, PartialEq)]
struct LoginGuestProps {
    err: Option<String>,
}

#[function_component(LoginGuest)]
fn login_guest(props: &LoginGuestProps) -> Html {
    let err_message = match &props.err {
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
