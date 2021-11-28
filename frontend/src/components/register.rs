use anyhow::Result;
use wasm_bindgen_futures::spawn_local;
use web_sys::{console::log_1, HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;
use yewdux::prelude::Dispatcher;
use yewdux_functional::{use_store, StoreRef};

use crate::{
    api::{
        auth::{register, RegisterPayload},
        user::get_user,
        ApiResult,
    },
    state::{AppState, AppStateStore, LoginErrorState, LoginErrorStateStore},
};

#[function_component(RegisterBox)]
pub fn register_box() -> Html {
    let invite_code_ref = NodeRef::default();
    let user_name_ref = NodeRef::default();
    let password_ref = NodeRef::default();
    let app_store = use_store::<AppStateStore>();
    let login_error_store = use_store::<LoginErrorStateStore>();

    let on_submit = {
        let user_name_ref = user_name_ref.clone();
        let password_ref = password_ref.clone();
        let invite_code_ref = invite_code_ref.clone();
        Callback::once(move |_| {
            spawn_local(async move {
                let user_name = user_name_ref.cast::<HtmlInputElement>().unwrap().value();
                let password = password_ref.cast::<HtmlTextAreaElement>().unwrap().value();
                let invite_code = invite_code_ref.cast::<HtmlInputElement>().unwrap().value();
                let payload = RegisterPayload::new(&user_name, &password, &invite_code);

                let register_res =
                    register_and_update(app_store, login_error_store, &payload).await;
                if let Err(err) = register_res {
                    log_1(&err.to_string().into());
                }
            })
        })
    };

    html! {
        <form class="box">
            <div class="content has-text-centered">
                <p class="title has-text-dark is-2">{"Register"}</p>
            </div>

            <div class="content">
                <p class="title has-text-dark is-5 level-left">{"User Name"}</p>
                <input class="input"
                type="text"
                size="50"
                autocomplete="username"
                ref={user_name_ref.clone()}/>
            </div>

            <div class="content">
                <p class="title has-text-dark is-5 level-left">{"Password"}</p>
                <input class="input"
                type="password"
                size="50"
                autocomplete="current-password"
                ref={password_ref.clone()}/>
            </div>

            <div class="content">
                <p class="title has-text-dark is-5 level-left">{"Invite Code"}</p>
                <input class="input"
                type="text"
                size="50"
                ref={invite_code_ref.clone()}/>
            </div>

            <a class="button is-primary" onclick={on_submit.clone()}>
                <strong>{"Register"}</strong>
            </a>
        </form>
    }
}

async fn register_and_update(
    app_state_store: StoreRef<AppStateStore>,
    login_error_store: StoreRef<LoginErrorStateStore>,
    payload: &RegisterPayload<'_>,
) -> Result<()> {
    let token = match register(payload).await? {
        ApiResult::Ok(token) => token,
        ApiResult::Err(err) => {
            login_error_store
                .dispatch()
                .reduce(|state| *state = LoginErrorState(Some(err.message)));
            return Ok(());
        }
    };

    let user = match get_user(&token.token).await? {
        ApiResult::Ok(user) => user,
        ApiResult::Err(err) => {
            login_error_store
                .dispatch()
                .reduce(|state| *state = LoginErrorState(Some(err.message)));
            return Ok(());
        }
    };

    app_state_store.dispatch().reduce(move |state| {
        let new_state = if &user.user_type == "tutor" {
            AppState::Tutor(token.token)
        } else {
            AppState::Admin(token.token, Vec::new())
        };

        *state = new_state;
    });

    Ok(())
}
