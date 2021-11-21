use std::rc::Rc;

use yew::prelude::*;
use yew_router::replace_route;
use yewdux_functional::use_store;

use crate::{
    components::{GuestNavBar, RegisterBox},
    state::{AppState, AppStateStore, GetState, LoginErrorState, LoginErrorStateStore},
    Route,
};

#[function_component(Register)]
pub fn register() -> Html {
    let store = use_store::<AppStateStore>();
    let app_state = store.get_state();
    let err_store = use_store::<LoginErrorStateStore>();
    let err_state = err_store.get_state();

    match app_state.as_ref() {
        AppState::Guest | AppState::RequestedGuest(_) => {
            html! {<RegisterGuest err={err_state}/>}
        }
        AppState::Tutor(_) | AppState::Admin(_) => {
            replace_route(Route::Index);
            html! {}
        }
    }
}

#[derive(Properties, PartialEq)]
struct RegisterGuestProps {
    err: Rc<LoginErrorState>,
}

#[function_component(RegisterGuest)]
fn register_guest(props: &RegisterGuestProps) -> Html {
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
                    <RegisterBox/>
                    {err_message}
                </div>
            </div>
        </section>
    }
}
