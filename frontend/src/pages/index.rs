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
        AppState::RequestedGuest(_) => {
            html! {}
        }
    }
}

#[function_component(IndexGuest)]
fn index_guest() -> Html {
    html! {
        <section class="hero is-info is-fullheight">
            <div class="hero-head">
                <GuestNavBar/>
            </div>

            <div class="hero-body container">
            <CreateTaskForm/>
            </div>
        </section>
    }
}
