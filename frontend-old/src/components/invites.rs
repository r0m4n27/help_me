use std::mem;

use anyhow::Result;
use wasm_bindgen_futures::spawn_local;
use web_sys::console::log_1;
use yew::prelude::*;
use yewdux::prelude::Dispatcher;
use yewdux_functional::{use_store, StoreRef};

use crate::{
    api::{
        admin::{create_invite, delete_invite, get_invites, Invite},
        ApiResult,
    },
    state::{AppState, AppStateStore, GetState, IndexErrorState, IndexErrorStateStore},
};

#[derive(PartialEq, Properties)]
pub struct InvitesProps {
    pub token: String,
    pub invites: Vec<Invite>,
}

#[function_component(Invites)]
pub fn invites(props: &InvitesProps) -> Html {
    let on_generate = {
        let token = props.token.clone();
        let app_store = use_store::<AppStateStore>();
        let err_store = use_store::<IndexErrorStateStore>();

        Callback::once(|_| {
            spawn_local(async move {
                if let Err(err) = create_invite_and_update(token, err_store, app_store).await {
                    log_1(&err.to_string().into())
                }
            })
        })
    };

    let token = props.token.clone();
    let app_store_deps = use_store::<AppStateStore>();
    let app_store = use_store::<AppStateStore>();
    let err_store = use_store::<IndexErrorStateStore>();

    use_effect_with_deps(
        |_| {
            spawn_local(async {
                if let Err(err) = refresh_invites(token, err_store, app_store).await {
                    log_1(&err.to_string().into())
                }
            });
            || {}
        },
        app_store_deps.get_state(),
    );

    let invites = props
        .invites
        .iter()
        .map(|invite| {
            let token = props.token.clone();
            let app_store = use_store::<AppStateStore>();
            let err_store = use_store::<IndexErrorStateStore>();

            html! {
                <tr key={invite.invite_code.as_str()}>
                    <th>{&invite.invite_code}</th>
                    <td>
                        <a class="has-text-link is-unselectable"
                            onclick={on_delete(token, invite.invite_code.clone(), err_store, app_store)}>
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
                        {"Invite Codes"}
                    </p>
                </div>
                <button class="button is-primary level-item has-text-centered"
                    onclick={on_generate}>
                    {"Generate Code"}
                </button>
            </div>
        </div>

        <table class="table">
            <tbody>
                {invites}
            </tbody>
        </table>
        </>
    }
}

async fn refresh_invites(
    token: String,
    err_store: StoreRef<IndexErrorStateStore>,
    app_store: StoreRef<AppStateStore>,
) -> Result<()> {
    match get_invites(&token).await? {
        ApiResult::Err(err) => err_store
            .dispatch()
            .reduce(|state| *state = IndexErrorState(Some(err.message))),
        ApiResult::Ok(mut invites) => app_store.dispatch().reduce(move |store| {
            if let AppState::Admin(_, old_invites, _) = store {
                mem::swap(&mut invites, old_invites)
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
            if let Err(err) =
                delete_invite_and_update(token, invite_code, err_store, app_store).await
            {
                log_1(&err.to_string().into())
            }
        })
    })
}

async fn delete_invite_and_update(
    token: String,
    invite_code: String,
    err_store: StoreRef<IndexErrorStateStore>,
    app_store: StoreRef<AppStateStore>,
) -> Result<()> {
    match delete_invite(&token, &invite_code).await? {
        ApiResult::Err(err) => err_store
            .dispatch()
            .reduce(|state| *state = IndexErrorState(Some(err.message))),
        ApiResult::Ok(_) => app_store.dispatch().reduce(move |state| {
            if let AppState::Admin(_, invites, _) = state {
                let mut old_invites = Vec::new();

                // A HashSet can't keep the insertion order and it would look weird
                // if a item is not inserted at the end of the list
                mem::swap(invites, &mut old_invites);
                let mut new_invites = old_invites
                    .into_iter()
                    .filter(|i| i.invite_code != invite_code)
                    .collect();

                mem::swap(invites, &mut new_invites)
            }
        }),
    }

    Ok(())
}

async fn create_invite_and_update(
    token: String,
    err_store: StoreRef<IndexErrorStateStore>,
    app_store: StoreRef<AppStateStore>,
) -> Result<()> {
    match create_invite(&token).await? {
        ApiResult::Err(err) => err_store
            .dispatch()
            .reduce(|state| *state = IndexErrorState(Some(err.message))),
        ApiResult::Ok(invite) => app_store.dispatch().reduce(|state| {
            if let AppState::Admin(_, ref mut invites, _) = state {
                invites.push(invite)
            }
        }),
    }

    Ok(())
}
