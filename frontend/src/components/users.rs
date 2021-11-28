use std::mem;

use anyhow::Result;
use wasm_bindgen_futures::spawn_local;
use web_sys::console::log_1;
use yew::prelude::*;
use yewdux::prelude::Dispatcher;
use yewdux_functional::{use_store, StoreRef};

use crate::{
    api::{
        user::{delete_user_admin, get_users_admin, User},
        ApiResult,
    },
    state::{AppState, AppStateStore, GetState, IndexErrorState, IndexErrorStateStore},
};

#[derive(PartialEq, Properties)]
pub struct UsersProps {
    pub token: String,
    pub users: Vec<User>,
}

#[function_component(Users)]
pub fn users(props: &UsersProps) -> Html {
    let token = props.token.clone();
    let app_store_deps = use_store::<AppStateStore>();
    let app_store = use_store::<AppStateStore>();
    let err_store = use_store::<IndexErrorStateStore>();

    use_effect_with_deps(
        |_| {
            spawn_local(async {
                if let Err(err) = refresh_users(token, err_store, app_store).await {
                    log_1(&err.to_string().into())
                }
            });
            || {}
        },
        app_store_deps.get_state(),
    );

    let users = props
        .users
        .iter()
        .map(|user| {
            let token = props.token.clone();
            let app_store = use_store::<AppStateStore>();
            let err_store = use_store::<IndexErrorStateStore>();

            html! {
                <tr key={user.user_name.as_str()}>
                    <th>{&user.user_name}</th>
                    <td>
                        <a class="has-text-link is-unselectable"
                            onclick={on_delete(token, user.user_name.clone(), err_store, app_store)}>
                            {"Delete"}
                        </a>
                    </td>
                </tr>
            }
        })
        .collect::<Html>();

    html! {
        <>
        <div class="level is-mobile">
            <div class="level-left">
                <div class="level-item">
                    <p class="title is-4 has-text-dark">
                        {"Users"}
                    </p>

                    // Use this so the Headline has the same position
                    // as 'Invites'
                    <button class="button is-primary level-item has-text-centered is-invisible">
                        {"Placeholder"}
                    </button>
                </div>
            </div>
        </div>

        <table class="table">
            <tbody>
                {users}
            </tbody>
        </table>
        </>
    }
}

async fn refresh_users(
    token: String,
    err_store: StoreRef<IndexErrorStateStore>,
    app_store: StoreRef<AppStateStore>,
) -> Result<()> {
    match get_users_admin(&token).await? {
        ApiResult::Err(err) => err_store
            .dispatch()
            .reduce(|state| *state = IndexErrorState(Some(err.message))),
        ApiResult::Ok(mut users) => app_store.dispatch().reduce(move |store| {
            if let AppState::Admin(_, _, old_users) = store {
                mem::swap(&mut users, old_users)
            }
        }),
    }
    Ok(())
}

fn on_delete(
    token: String,
    invite_code: String,
    err_store: StoreRef<IndexErrorStateStore>,
    app_store: StoreRef<AppStateStore>,
) -> Callback<MouseEvent> {
    Callback::once(|_| {
        spawn_local(async move {
            if let Err(err) = delete_user_and_update(token, invite_code, err_store, app_store).await
            {
                log_1(&err.to_string().into())
            }
        })
    })
}

async fn delete_user_and_update(
    token: String,
    user_name: String,
    err_store: StoreRef<IndexErrorStateStore>,
    app_store: StoreRef<AppStateStore>,
) -> Result<()> {
    match delete_user_admin(&token, &user_name).await? {
        ApiResult::Err(err) => err_store
            .dispatch()
            .reduce(|state| *state = IndexErrorState(Some(err.message))),
        ApiResult::Ok(_) => app_store.dispatch().reduce(move |state| {
            if let AppState::Admin(_, _, users) = state {
                let mut old_users = Vec::new();

                // A HashSet can't keep the insertion order and it would look weird
                // if a item is not inserted at the end of the list
                mem::swap(users, &mut old_users);
                let mut new_users = old_users
                    .into_iter()
                    .filter(|u| u.user_name != user_name)
                    .collect();

                mem::swap(users, &mut new_users)
            }
        }),
    }

    Ok(())
}
