use wasm_bindgen_futures::spawn_local;
use web_sys::console::log_1;
use yew::prelude::*;
use yewdux::prelude::Dispatcher;

use crate::{
    api::{submit_request, CreateTaskPayload},
    state::{app_state_store, AppState},
};

#[function_component(CreateTaskForm)]
pub fn create_task_form() -> Html {
    let title = use_state(String::new);
    let description = use_state(String::new);
    let store = app_state_store();

    let on_title = {
        let title = title.clone();

        Callback::from(move |event: InputEvent| {
            if let Some(data) = event.data() {
                title.set((*title).clone() + &data);
            }
        })
    };

    let on_description = {
        let description = description.clone();

        Callback::from(move |event: InputEvent| {
            if let Some(data) = event.data() {
                description.set((*description).clone() + &data)
            }
        })
    };

    let on_submit = {
        Callback::once(move |_| {
            spawn_local(async move {
                let payload = CreateTaskPayload::new(title.to_string(), description.to_string());
                let task = submit_request(payload).await;
                log_1(&format!("{:?}", task).into());

                store
                    .dispatch()
                    .reduce(|app| *app = AppState::RequestedGuest(task));
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
                <input class="input" type="text" size="50" oninput={on_title}/>
            </div>

            <div class="content">
                <p class="title has-text-dark is-5 level-left">{"Description"}</p>
                <textarea class="textarea has-fixed-size"
                    type="textarea"
                    size="50"
                    oninput={on_description}/>
            </div>

            <button class="button is-primary" onclick={on_submit}>
                <strong>{"Submit"}</strong>
            </button>
        </div>
    }
}
