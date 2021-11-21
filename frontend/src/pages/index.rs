use std::rc::Rc;

use yew::prelude::*;
use yewdux_functional::use_store;

use crate::{
    api::tasks::Task,
    components::{NavBar, RequestedTask, SubmitTask},
    state::{AppState, AppStateStore, GetState, IndexErrorState, IndexErrorStateStore},
};

use super::ErrorMessage;

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
        AppState::Admin(_) => html! {
            <IndexAdmin err={err_state}/>
        },
    }
}

#[derive(Properties, PartialEq)]
struct IndexGuestProps {
    err: Rc<IndexErrorState>,
}

#[function_component(IndexGuest)]
fn index_guest(props: &IndexGuestProps) -> Html {
    html! {
        <section class="hero is-info is-fullheight">
            <div class="hero-head">
                <NavBar logged_in={false}/>
            </div>

            <div class="hero-body section">
                <div class="container">
                    <SubmitTask/>
                    <ErrorMessage err={props.err.0.clone()}/>
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
    html! {
        <section class="hero is-info is-fullheight">
            <div class="hero-head">
                <NavBar logged_in={false}/>
            </div>

            <div class="hero-body section">
                <div class="container">
                    <RequestedTask task={props.task.clone()}/>
                    <ErrorMessage err={props.err.0.clone()}/>
                </div>
            </div>
        </section>
    }
}

#[derive(Properties, PartialEq)]
struct IndexAdminProps {
    err: Rc<IndexErrorState>,
}

#[function_component(IndexAdmin)]
fn index_admin(props: &IndexAdminProps) -> Html {
    html! {
        <section class="hero is-info is-fullheight">
        <div class="hero-head">
            <NavBar logged_in={true}/>
        </div>

        <div class="hero-body section">
            <div class="container">
                <SubmitTask/>
                <ErrorMessage err={props.err.0.clone()}/>
            </div>
        </div>
    </section>
    }
}
