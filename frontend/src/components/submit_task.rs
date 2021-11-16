use anyhow::Result;
use wasm_bindgen_futures::spawn_local;
use web_sys::{console::log_1, HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;
use yewdux::prelude::Dispatcher;
use yewdux_functional::{use_store, StoreRef};

use super::edit_task::EditTask;
use crate::{
    api::{tasks::submit_request, ApiResult},
    state::{AppState, AppStateStore, IndexErrorState, IndexErrorStateStore},
};

#[function_component(SubmitTask)]
pub fn submit_task() -> Html {
    let title_ref = NodeRef::default();
    let description_ref = NodeRef::default();
    let store = use_store::<AppStateStore>();
    let err_store = use_store::<IndexErrorStateStore>();

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

                if let Err(err) =
                    submit_request_and_update(&title, &description, store, err_store).await
                {
                    log_1(&err.to_string().into())
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

async fn submit_request_and_update(
    title: &str,
    description: &str,
    app_store: StoreRef<AppStateStore>,
    err_store: StoreRef<IndexErrorStateStore>,
) -> Result<()> {
    match submit_request(title, description).await? {
        ApiResult::Ok(task) => app_store
            .dispatch()
            .reduce(|state| *state = AppState::RequestedGuest(task)),
        ApiResult::Err(err) => err_store
            .dispatch()
            .reduce(|state| *state = IndexErrorState(Some(err.message))),
    }

    Ok(())
}
