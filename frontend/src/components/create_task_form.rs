use wasm_bindgen_futures::spawn_local;
use web_sys::{console::log_1, HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;
use yewdux::prelude::Dispatcher;

use crate::{
    api::{tasks::submit_request, ApiResult},
    state::{app_state_store, AppState},
};

#[function_component(CreateTaskForm)]
pub fn create_task_form() -> Html {
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
        <div class="box">
            <div class="content has-text-centered">
                <p class="title has-text-dark is-2">{"Submit Request"}</p>
            </div>

            <div class="content">
                <p class="title has-text-dark is-5 level-left">{"Title"}</p>
                <input class="input" type="text" size="50" ref={title_ref.clone()}/>
            </div>

            <div class="content">
                <p class="title has-text-dark is-5 level-left">{"Description"}</p>
                <textarea class="textarea has-fixed-size"
                    type="textarea"
                    size="50"
                    ref={description_ref.clone()}/>
            </div>

            <button class="button is-primary" onclick={on_submit.clone()}>
                <strong>{"Submit"}</strong>
            </button>
        </div>
    }
}
