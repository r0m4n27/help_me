use anyhow::Result;
use wasm_bindgen_futures::spawn_local;
use web_sys::{console::log_1, HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;
use yewdux::prelude::{Dispatcher, PersistentStore};
use yewdux_functional::{use_store, StoreRef};

use crate::{
    api::{
        tasks::{get_task, resolve_request, update_task, Task},
        ApiResult,
    },
    state::{AppState, AppStateStore},
};

use super::{edit_task::EditTask, view_task::ViewTask};

// Sadly yew can't have generic liftime parameters
#[derive(Properties, PartialEq, Clone)]
pub struct RequestedTaskProps {
    pub task: Task,
}

#[function_component(RequestedTask)]
pub fn requested_task(props: &RequestedTaskProps) -> Html {
    let store = use_store::<AppStateStore>();
    let editing = use_state(|| false);

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

    let on_edit = {
        let editing = editing.clone();
        Callback::from(move |_| editing.set(!*editing))
    };

    let on_cancel = {
        let editing = editing.clone();
        Callback::from(move |_| editing.set(false))
    };

    let title_ref = NodeRef::default();
    let description_ref = NodeRef::default();
    let on_confirm = {
        // This is what happens if if you can't speciy lifetimes
        // and almost everything is reference counted
        let title_ref = title_ref.clone();
        let description_ref = description_ref.clone();
        let props_task = props.task.clone();

        let editing = editing.clone();
        let store = use_store::<AppStateStore>();

        Callback::once(move |_| {
            spawn_local(async move {
                let title = title_ref.cast::<HtmlInputElement>().unwrap().value();
                let description = description_ref
                    .cast::<HtmlTextAreaElement>()
                    .unwrap()
                    .value();

                let result = update_task(&props_task.id, &title, &description).await;

                match result {
                    Ok(result) => store.dispatch().reduce(move |app| {
                        *app = match result {
                            ApiResult::Ok(_) => {
                                let task = Task {
                                    title,
                                    body: description,
                                    ..props_task
                                };
                                editing.set(false);

                                AppState::RequestedGuest(task, None)
                            }
                            ApiResult::Err(err) => AppState::RequestedGuest(props_task, Some(err)),
                        }
                    }),
                    Err(err) => log_1(&err.to_string().into()),
                }
            })
        })
    };

    if *editing {
        html! {
        <EditTask header={"Edit Request".to_string()}
            start_title={Some(props.task.title.clone())}
            start_description={Some(props.task.body.clone())}
            title_ref={title_ref}
            description_ref={description_ref}>
            <div class="buttons">
                <button class="button is-danger" onclick={on_cancel}>
                    <strong>{"Cancel"}</strong>
                </button>

                <button class="button is-primary" onclick={on_confirm}>
                    <strong>{"Confirm"}</strong>
                </button>
            </div>
        </EditTask>
        }
    } else {
        html! {
        <ViewTask header={"Request Submitted".to_string()}
            title={props.task.title.clone()}
            description={props.task.body.clone()}>
            <div class="buttons">
                <button class="button is-danger" onclick={on_revoke}>
                    <strong>{"Resolve"}</strong>
                </button>

                <button class="button is-info" onclick={on_edit}>
                    <strong>{"Edit"}</strong>
                </button>
            </div>
        </ViewTask>
        }
    }
}

async fn resolve_request_and_update(
    store: &StoreRef<PersistentStore<AppState>>,
    task: Task,
) -> Result<()> {
    let api_error = match resolve_request(&task.id).await? {
        ApiResult::Ok(_) => {
            store
                .dispatch()
                .reduce(|state| *state = AppState::Guest(None));
            return Ok(());
        }
        ApiResult::Err(err) => err,
    };

    match get_task(&task.id).await? {
        ApiResult::Ok(task) => {
            if task.state == "done" {
                store
                    .dispatch()
                    .reduce(|state| *state = AppState::Guest(Some(api_error)))
            } else {
                store
                    .dispatch()
                    .reduce(|state| *state = AppState::RequestedGuest(task, Some(api_error)))
            }
        }
        ApiResult::Err(err) => store
            .dispatch()
            .reduce(|state| *state = AppState::Guest(Some(err))),
    };

    Ok(())
}
