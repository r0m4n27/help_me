use anyhow::Result;
use wasm_bindgen_futures::spawn_local;
use web_sys::console::log_1;
use yew::prelude::*;
use yewdux::prelude::{Dispatcher, PersistentStore};
use yewdux_functional::StoreRef;

use crate::{
    api::{get_task, resolve_request, ApiResult, Task},
    state::{app_state_store, AppState},
};

// Sadly yew can't have generic liftime parameters
#[derive(Properties, PartialEq, Clone)]
pub struct RequestedTaskProps {
    pub task: Task,
}

#[function_component(RequestedTask)]
pub fn requested_task(props: &RequestedTaskProps) -> Html {
    let store = app_state_store();

    let on_revoke = {
        let props = props.clone();
        Callback::once(move |_| {
            spawn_local(async move {
                if let Err(err) = resolve_request_and_update(&store, props.task).await {
                    log_1(&err.to_string().into())
                }
            })
        })
    };

    html! {
        <div class="box">
            <div class="content has-text-centered">
                <p class="title has-text-dark is-2">{"Request Submitted"}</p>
            </div>

            <div class="content">
                <p class="is-size-4 has-text-weight-bold">{props.task.title.clone()}</p>
            </div>

            <div class="content">
                <p class="is-size-5">{props.task.body.clone()}</p>
            </div>

            <div class="columns">
                <div class="column is-1">
                    <button class="button is-danger" onclick={on_revoke}>
                        <strong>{"Resolve"}</strong>
                    </button>
                </div>

                <div class="column is-1">
                    <button class="button is-info">
                        <strong>{"Edit"}</strong>
                    </button>
                </div>
            </div>
        </div>
    }
}

async fn resolve_request_and_update(
    store: &StoreRef<PersistentStore<AppState>>,
    task: Task,
) -> Result<()> {
    let api_error = match resolve_request(&task.id).await? {
        ApiResult::Ok(_) => {
            store.dispatch().reduce(|state| *state = AppState::Guest);
            return Ok(());
        }
        ApiResult::Err(err) => err,
    };

    match get_task(&task.id).await? {
        ApiResult::Ok(task) => {
            if task.state == "done" {
                store
                    .dispatch()
                    .reduce(|state| *state = AppState::GuestErr(api_error))
            } else {
                store
                    .dispatch()
                    .reduce(|state| *state = AppState::RequestedGuestErr(task, api_error))
            }
        }
        ApiResult::Err(err) => store
            .dispatch()
            .reduce(|state| *state = AppState::GuestErr(err)),
    };

    Ok(())
}
