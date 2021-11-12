use yew::prelude::*;

use crate::{
    components::{CreateTaskForm, GuestNavBar},
    state::{app_state_store, AppState},
};

#[function_component(Index)]
pub fn index() -> Html {
    let store = app_state_store();
    let app_state = store.state().map(|s| s.as_ref()).unwrap_or_default();

    match app_state {
        AppState::Guest => {
            html! {<IndexGuest/>}
        }
        AppState::GuestErr(err) => html! {
            <IndexGuest err={err.clone().message}/>
        },
        AppState::RequestedGuest(_) => {
            html! {}
        }
    }
}

#[derive(Properties, PartialEq)]
struct IndexGuestProps {
    err: Option<String>,
}

#[function_component(IndexGuest)]
fn index_guest(props: &IndexGuestProps) -> Html {
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

            <div class="hero-body container">
                <div>
                    <CreateTaskForm/>
                    {err_message}
                </div>
            </div>
        </section>
    }
}
