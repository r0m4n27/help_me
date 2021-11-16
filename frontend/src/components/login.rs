use anyhow::Result;
use wasm_bindgen_futures::spawn_local;
use web_sys::{console::log_1, HtmlInputElement, HtmlTextAreaElement};
use yew::prelude::*;
use yewdux::prelude::{BasicStore, Dispatcher};
use yewdux_functional::{use_store, StoreRef};

use crate::{
    api::{auth::login, user::get_user, ApiResult},
    state::{AppState, AppStateStore},
};

#[derive(Clone)]
pub struct LoginError(pub String);

pub type LoginErrorStore = BasicStore<Option<LoginError>>;

#[function_component(LoginBox)]
pub fn login_box() -> Html {
    let user_name_ref = NodeRef::default();
    let password_ref = NodeRef::default();
    let app_store = use_store::<AppStateStore>();
    let login_error_store = use_store::<LoginErrorStore>();

    let on_submit = {
        let user_name_ref = user_name_ref.clone();
        let password_ref = password_ref.clone();
        Callback::once(move |_| {
            spawn_local(async move {
                let user_name = user_name_ref.cast::<HtmlInputElement>().unwrap().value();
                let password = password_ref.cast::<HtmlTextAreaElement>().unwrap().value();
                let login_res =
                    login_and_update(app_store, login_error_store, &user_name, &password).await;
                if let Err(err) = login_res {
                    log_1(&err.to_string().into());
                }
            })
        })
    };

    html! {
        <form class="box">
            <div class="content has-text-centered">
                <p class="title has-text-dark is-2">{"Login"}</p>
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

            <a class="button is-primary" onclick={on_submit.clone()}>
                <strong>{"Login"}</strong>
            </a>
        </form>
    }
}

async fn login_and_update(
    app_state_store: StoreRef<AppStateStore>,
    login_error_store: StoreRef<LoginErrorStore>,
    user_name: &str,
    password: &str,
) -> Result<()> {
    let token = match login(user_name, password).await? {
        ApiResult::Ok(token) => token,
        ApiResult::Err(err) => {
            login_error_store
                .dispatch()
                .reduce(|state| *state = Some(LoginError(err.message)));
            return Ok(());
        }
    };

    let user = match get_user(&token.token).await? {
        ApiResult::Ok(user) => user,
        ApiResult::Err(err) => {
            login_error_store
                .dispatch()
                .reduce(|state| *state = Some(LoginError(err.message)));
            return Ok(());
        }
    };

    app_state_store.dispatch().reduce(move |state| {
        let new_state = if &user.user_type == "tutor" {
            AppState::Tutor(token.token)
        } else {
            AppState::Admin(token.token)
        };

        *state = new_state;
    });

    Ok(())
}
