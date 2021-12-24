use std::rc::Rc;

use anyhow::Result;
use wasm_bindgen_futures::spawn_local;
use web_sys::console::log_1;
use yew::prelude::*;
use yew_router::{history::History, hooks::use_history};
use yewdux::prelude::Dispatcher;
use yewdux_functional::{use_store, StoreRef};

use super::{on_init, ErrorMessage, UninitialisedView};
use crate::{
    api::{tasks::get_task, ApiResult},
    components::{NavBar, TutorTask},
    state::{
        AppState, AppStateStore, GetState, TaskErrorState, TaskErrorStateStore, TaskState,
        TaskStateStore,
    },
    Route,
};

#[derive(PartialEq, Properties)]
pub struct TaskProps {
    pub task_id: String,
}

#[function_component(Task)]
pub fn task(props: &TaskProps) -> Html {
    let store = use_store::<AppStateStore>();
    let app_state = store.get_state();
    let err_store = use_store::<TaskErrorStateStore>();
    let err_state = err_store.get_state();
    let history = use_history().unwrap();

    {
        let store = use_store::<AppStateStore>();
        let app_state = store.get_state();

        on_init(move || match app_state.as_ref() {
            AppState::Tutor(_, _) => {}
            _ => history.replace(Route::Index),
        });
    }

    match app_state.as_ref() {
        AppState::Tutor(token, _) => html! {
            <TaskContent err={err_state}
            task_id={props.task_id.clone()}
            token={token.clone()}/>
        },
        _ => {
            html! {<UninitialisedView/>}
        }
    }
}

#[derive(Properties, PartialEq)]
struct TaskContentProps {
    err: Rc<TaskErrorState>,
    task_id: String,
    token: String,
}

#[function_component(TaskContent)]
fn task_content(props: &TaskContentProps) -> Html {
    let task_id = props.task_id.clone();
    let store = use_store::<TaskStateStore>();
    let err_store = use_store::<TaskErrorStateStore>();

    let task_state_store = use_store::<TaskStateStore>();
    let task_state = task_state_store.get_state();

    use_effect_with_deps(
        |_| {
            spawn_local(async {
                if let Err(err) = fetch_task(task_id, err_store, store).await {
                    log_1(&err.to_string().into())
                }
            });
            || {}
        },
        true,
    );

    let site_content = match &task_state.as_ref().0 {
        Some(task) => html! {
            <TutorTask task={task.clone()} token={props.token.clone()}/>
        },
        None => html! {},
    };

    html! {
    <section class="hero is-info is-fullheight">
        <div class="hero-head">
            <NavBar/>
        </div>

        <div class="hero-body section">
            <div class="container">
                {site_content}
                <ErrorMessage err={props.err.0.clone()}/>
            </div>
        </div>
    </section>
    }
}

async fn fetch_task(
    task_id: String,
    err_store: StoreRef<TaskErrorStateStore>,
    task_store: StoreRef<TaskStateStore>,
) -> Result<()> {
    match get_task(&task_id).await? {
        ApiResult::Err(err) => err_store
            .dispatch()
            .reduce(|state| *state = TaskErrorState(Some(err.message))),
        ApiResult::Ok(task) => task_store
            .dispatch()
            .reduce(move |store| *store = TaskState(Some(task))),
    }
    Ok(())
}
