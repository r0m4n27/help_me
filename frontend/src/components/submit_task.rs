use wasm_bindgen_futures::spawn_local;
use web_sys::{console::log_1, HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;
use yewdux::prelude::Dispatcher;

use super::edit_task::EditTask;
use crate::{
    api::{tasks::submit_request, ApiResult},
    state::{app_state_store, AppState},
};

#[function_component(SubmitTask)]
pub fn submit_task() -> Html {
    let title_ref = NodeRef::default();
    let description_ref = NodeRef::default();
    let store = app_state_store();

    let on_submit = {
        let title_ref = title_ref.clone();
        let description_ref = description_ref.clone();
        Callback::once(move |_| {
            spawn_local(async move {
                let title = title_ref.cast::<HtmlInputElement>().unwrap().value();
                let description = description_ref
                    .cast::<HtmlTextAreaElement>()
                    .unwrap()
                    .value();

                let task = submit_request(&title, &description).await;

                match task {
                    Ok(task) => store.dispatch().reduce(|app| {
                        *app = match task {
                            ApiResult::Ok(task) => AppState::RequestedGuest(task, None),
                            ApiResult::Err(err) => AppState::Guest(Some(err)),
                        }
                    }),
                    Err(err) => log_1(&err.to_string().into()),
                }
            })
        })
    };

    html! {
        <EditTask header={"Submit Request".to_string()}
            start_title={None::<String>}
            start_description={None::<String>}
            title_ref={title_ref}
            description_ref={description_ref}>
            <button class="button is-primary" onclick={on_submit.clone()}>
                <strong>{"Submit"}</strong>
            </button>
        </EditTask>
    }
}
