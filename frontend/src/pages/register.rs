use std::rc::Rc;

use yew::prelude::*;
use yew_router::replace_route;
use yewdux_functional::use_store;

use crate::{
    components::{NavBar, RegisterBox},
    state::{AppState, AppStateStore, GetState, RegisterErrorState, RegisterErrorStateStore},
    Route,
};

use super::ErrorMessage;

#[function_component(Register)]
pub fn register() -> Html {
    let store = use_store::<AppStateStore>();
    let app_state = store.get_state();
    let err_store = use_store::<RegisterErrorStateStore>();
    let err_state = err_store.get_state();

    match app_state.as_ref() {
        AppState::Guest | AppState::RequestedGuest(_) => {
            html! {<RegisterGuest err={err_state}/>}
        }
        AppState::Tutor(_, _) | AppState::Admin(..) => {
            replace_route(Route::Index);
            html! {}
        }
    }
}

#[derive(Properties, PartialEq)]
struct RegisterGuestProps {
    err: Rc<RegisterErrorState>,
}

#[function_component(RegisterGuest)]
fn register_guest(props: &RegisterGuestProps) -> Html {
    html! {
        <section class="hero is-info is-fullheight">
            <div class="hero-head">
                <NavBar/>
            </div>

            <div class="hero-body section">
                <div class="container">
                    <RegisterBox/>
                    <ErrorMessage err={props.err.0.clone()}/>
                </div>
            </div>
        </section>
    }
}
