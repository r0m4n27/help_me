use std::rc::Rc;

use yew::prelude::*;
use yewdux_functional::use_store;

use crate::{
    api::tasks::Task,
    components::{GuestNavBar, RequestedTask, SubmitTask},
    state::{AppState, AppStateStore, GetState, IndexErrorState, IndexErrorStateStore},
};

#[function_component(Index)]
pub fn index() -> Html {
    let store = use_store::<AppStateStore>();
    let app_state = store.get_state();
    let err_store = use_store::<IndexErrorStateStore>();
    let err_state = err_store.get_state();

    match app_state.as_ref() {
        AppState::Guest => {
            html! {<IndexGuest err={err_state}/>}
        }
        AppState::RequestedGuest(task) => {
            html! {<IndexGuestRequested task={task.clone()} err={err_state}/>}
        }
        AppState::Tutor(_) => html! {},
        AppState::Admin(_) => html! {},
    }
}

#[derive(Properties, PartialEq)]
struct IndexGuestProps {
    err: Rc<IndexErrorState>,
}

#[function_component(IndexGuest)]
fn index_guest(props: &IndexGuestProps) -> Html {
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
                    <SubmitTask/>
                    {err_message}
                </div>
            </div>
        </section>
    }
}

#[derive(PartialEq, Properties)]
pub struct IndexGuestRequestedProps {
    task: Task,
    err: Rc<IndexErrorState>,
}

#[function_component(IndexGuestRequested)]
fn index_guest_requested(props: &IndexGuestRequestedProps) -> Html {
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
                    <RequestedTask task={props.task.clone()}/>
                    {err_message}
                </div>
            </div>
        </section>
    }
}
