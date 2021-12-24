use anyhow::Result;
use wasm_bindgen_futures::spawn_local;
use web_sys::console::log_1;
use yew::prelude::*;
use yew_router::{history::History, hooks::use_history};
use yewdux::prelude::Dispatcher;
use yewdux_functional::{use_store, StoreRef};

use crate::{
    api::{
        tasks::{finish_task, process_task, Task},
        ApiResult,
    },
    state::{TaskErrorState, TaskErrorStateStore, TaskStateStore},
    Route,
};

use super::view_task::ViewTask;

#[derive(PartialEq, Properties)]
pub struct TutorTaskProps {
    pub token: String,
    pub task: Task,
}

#[function_component(TutorTask)]
pub fn tutor_task(props: &TutorTaskProps) -> Html {
    let on_start = {
        let token = props.token.clone();
        let task_id = props.task.id.clone();
        let task_store = use_store::<TaskStateStore>();
        let err_store = use_store::<TaskErrorStateStore>();

        Callback::once(|_| {
            spawn_local(async {
                if let Err(err) = process_and_update(token, task_id, err_store, task_store).await {
                    log_1(&err.to_string().into())
                }
            })
        })
    };

    let on_finish = {
        let token = props.token.clone();
        let task_id = props.task.id.clone();
        let task_store = use_store::<TaskStateStore>();
        let err_store = use_store::<TaskErrorStateStore>();
        let history = use_history().unwrap();

        Callback::once(move |_| {
            spawn_local(async move {
                if let Err(err) = finish_and_update(token, task_id, err_store, task_store).await {
                    log_1(&err.to_string().into())
                } else {
                    history.replace(Route::Index)
                }
            })
        })
    };

    let button = if props.task.state.as_str() == "pending" {
        html! {
            <button class="button is-info"
                onclick={on_start}>
                {"Start"}
            </button>
        }
    } else {
        html! {
            <button class="button is-primary"
            onclick={on_finish}>
                {"Finish"}
            </button>
        }
    };

    html! {
        <ViewTask header={"Process Task".to_string()}
            title={props.task.title.clone()}
            description={props.task.body.clone()}>
            {button}
        </ViewTask>
    }
}

async fn process_and_update(
    token: String,
    task_id: String,
    err_store: StoreRef<TaskErrorStateStore>,
    task_store: StoreRef<TaskStateStore>,
) -> Result<()> {
    match process_task(&token, &task_id).await? {
        ApiResult::Err(err) => err_store
            .dispatch()
            .reduce(|state| *state = TaskErrorState(Some(err.message))),
        ApiResult::Ok(_) => task_store.dispatch().reduce(|state| {
            if let Some(ref mut task) = state.0 {
                task.state = "doing".to_string()
            }
        }),
    };

    Ok(())
}

async fn finish_and_update(
    token: String,
    task_id: String,
    err_store: StoreRef<TaskErrorStateStore>,
    task_store: StoreRef<TaskStateStore>,
) -> Result<()> {
    match finish_task(&token, &task_id).await? {
        ApiResult::Err(err) => err_store
            .dispatch()
            .reduce(|state| *state = TaskErrorState(Some(err.message))),
        ApiResult::Ok(_) => task_store.dispatch().reduce(|state| {
            if let Some(ref mut task) = state.0 {
                task.state = "done".to_string()
            }
        }),
    };

    Ok(())
}
