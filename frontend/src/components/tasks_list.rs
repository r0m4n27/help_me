use std::{collections::HashMap, mem};

use anyhow::Result;
use wasm_bindgen_futures::spawn_local;
use web_sys::console::log_1;
use yew::prelude::*;
use yewdux::prelude::Dispatcher;
use yewdux_functional::{use_store, StoreRef};

use crate::{
    api::{
        tasks::{get_tasks, Task},
        ApiResult,
    },
    state::{AppState, AppStateStore, GetState, IndexErrorState, IndexErrorStateStore},
};

#[derive(PartialEq, Properties)]
pub struct TasksListProps {
    pub token: String,
    pub tasks: HashMap<String, Task>,
}

#[function_component(TasksList)]
pub fn tasks_list(props: &TasksListProps) -> Html {
    let token = props.token.clone();
    let app_store_deps = use_store::<AppStateStore>();
    let app_store = use_store::<AppStateStore>();
    let err_store = use_store::<IndexErrorStateStore>();

    use_effect_with_deps(
        |_| {
            spawn_local(async {
                if let Err(err) = refresh_tasks(token, err_store, app_store).await {
                    log_1(&err.to_string().into())
                }
            });
            || {}
        },
        app_store_deps.get_state(),
    );

    let tasks = props
        .tasks
        .values()
        .map(|task| {
            html! {
                <tr key={task.id.as_str()}>
                <th class="content">
                        <p class="is-bold">
                            <strong>
                                {&task.state.to_uppercase()}
                            </strong>
                        </p>
                    </th>
                    <td>
                        <a class="has-text-link is-unselectable is-hidden-touch"
                            style="
                            display:inline-block;
                            white-space: nowrap;
                            overflow: hidden;
                            text-overflow: ellipsis;
                            max-width: 80ch;">
                            {&task.title}
                        </a>
                    </td>
                    <td>
                        <a class="has-text-link is-unselectable is-hidden-desktop"
                            style="
                            display:inline-block;
                            white-space: nowrap;
                            overflow: hidden;
                            text-overflow: ellipsis;
                            max-width: 20ch;">
                            {&task.title}
                        </a>
                    </td>

                </tr>
            }
        })
        .collect::<Html>();

    html! {
        <div class="box">
            <div class="content has-text-centered">
                <p class="title has-text-dark is-2">{"Requests"}</p>
            </div>

            <div class="table-container level">
                <table class="table level-item">
                    <tbody>
                        {tasks}
                    </tbody>
                </table>
            </div>
        </div>
    }
}

async fn refresh_tasks(
    token: String,
    err_store: StoreRef<IndexErrorStateStore>,
    app_store: StoreRef<AppStateStore>,
) -> Result<()> {
    match get_tasks(&token).await? {
        ApiResult::Err(err) => err_store
            .dispatch()
            .reduce(|state| *state = IndexErrorState(Some(err.message))),
        ApiResult::Ok(tasks) => app_store.dispatch().reduce(move |store| {
            if let AppState::Tutor(_, ref mut old_tasks) = store {
                let mut tasks_mapped = tasks
                    .into_iter()
                    .map(|task| (task.id.clone(), task))
                    .collect();

                mem::swap(old_tasks, &mut tasks_mapped)
            }
        }),
    }
    Ok(())
}
