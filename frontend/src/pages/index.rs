use yew::prelude::*;

use crate::{
    api::tasks::Task,
    components::{GuestNavBar, RequestedTask, SubmitTask},
    state::{app_state_store, AppState},
};

#[function_component(Index)]
pub fn index() -> Html {
    let store = app_state_store();
    let app_state = store.state().map(|s| s.as_ref()).unwrap_or_default();

    match app_state {
        AppState::Guest(err) => {
            html! {<IndexGuest err={err.clone().map(|e| e.message)}/>}
        }
        AppState::RequestedGuest(task, err) => {
            html! {<IndexGuestRequested task={task.clone()} err={err.clone().map(|e| e.message)}/>}
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
    err: Option<String>,
}

#[function_component(IndexGuestRequested)]
fn index_guest_requested(props: &IndexGuestRequestedProps) -> Html {
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
                    <RequestedTask task={props.task.clone()}/>
                    {err_message}
                </div>
            </div>
        </section>
    }
}
