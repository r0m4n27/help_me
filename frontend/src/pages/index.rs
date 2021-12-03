use std::rc::Rc;

use yew::prelude::*;

use yewdux_functional::use_store;

use crate::{
    api::{admin::Invite, tasks::Task, user::User},
    components::{Invites, NavBar, RequestedTask, SubmitTask, TasksList, Users},
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
        AppState::Tutor(token, tasks) => html! {
            <IndexTutor err={err_state}
                token={token.clone()}
                tasks={tasks.clone()}/>
        },
        AppState::Admin(token, invites, users) => html! {
            <IndexAdmin err={err_state}
                token={token.clone()}
                invites={invites.clone()}
                users={users.clone()}/>
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
                <NavBar/>
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
                <NavBar/>
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
    token: String,
    invites: Vec<Invite>,
    users: Vec<User>,
    err: Rc<IndexErrorState>,
}

#[function_component(IndexAdmin)]
fn index_admin(props: &IndexAdminProps) -> Html {
    html! {
        <section class="hero is-info is-fullheight">
        <div class="hero-head">
            <NavBar/>
        </div>

        <div class="hero-body section">
            <div class="container">
                <div class="box">
                    <div class="columns">
                        <div class="column">
                            <Invites token={props.token.clone()} invites={props.invites.clone()}/>
                        </div>
                        <div class="column">
                            <Users token={props.token.clone()} users={props.users.clone()}/>
                        </div>
                    </div>
                </div>
                <ErrorMessage err={props.err.0.clone()}/>
            </div>
        </div>
    </section>
    }
}

#[derive(Properties, PartialEq)]
struct IndexTutorProps {
    token: String,
    tasks: Vec<Task>,
    err: Rc<IndexErrorState>,
}

#[function_component(IndexTutor)]
fn index_tutor(props: &IndexTutorProps) -> Html {
    html! {
        <section class="hero is-info is-fullheight">
        <div class="hero-head">
            <NavBar/>
        </div>

        <div class="hero-body section">
            <div class="container">
                <TasksList tasks={props.tasks.clone()} token={props.token.clone()}/>
                <ErrorMessage err={props.err.0.clone()}/>
            </div>
        </div>
    </section>
    }
}
