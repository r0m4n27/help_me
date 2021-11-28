use anyhow::Result;
use wasm_bindgen_futures::spawn_local;
use web_sys::console::log_1;
use yew::prelude::*;
use yew_router::components::Link;
use yewdux::prelude::Dispatcher;
use yewdux_functional::{use_store, StoreRef};

use crate::{
    api::{auth::log_out, ApiResult},
    state::{AppState, AppStateStore, GetState, IndexErrorState, IndexErrorStateStore},
    Route,
};

#[function_component(NavBar)]
pub fn nav_bar() -> Html {
    let menu_expanded = use_state(|| false);

    let on_menu = {
        let menu_expanded = menu_expanded.clone();
        Callback::from(move |_| menu_expanded.set(!*menu_expanded))
    };

    let expanded_class = if *menu_expanded {
        Some("is-active")
    } else {
        None
    };

    html! {
        <nav class="navbar" role="navigation" aria-label="main navigation">
            <NavBarBrand expanded_class={expanded_class} on_menu={on_menu}/>

            <NavBarItems expanded_class={expanded_class}/>
        </nav>
    }
}

#[derive(PartialEq, Properties)]
struct NavBarBrandProps {
    expanded_class: Option<String>,
    on_menu: Callback<MouseEvent>,
}

#[function_component(NavBarBrand)]
fn nav_bar_brand(props: &NavBarBrandProps) -> Html {
    html! {
        <div class="navbar-brand">
            <Link<Route> route={Route::Index} classes={classes!("navbar-item")}>
                <p class="title is-unselectable">{"Help Me"}</p>
            </Link<Route>>

            <div class="navbar-item is-hidden-desktop">
                <a class="has-text-light is-flex is-align-items-center"
                    href="http://github.com/r0m4n27"
                    target="_blank">
                    <i class="fab fa-github-alt is-size-4"/>
                </a>
            </div>

            <a role="button"
                class={classes!("navbar-burger", &props.expanded_class)}
                aria-label="menu" aria-expanded="false"
                data-target="navbarBasic"
                onclick={&props.on_menu}>

                <span aria-hidden="true"></span>
                <span aria-hidden="true"></span>
                <span aria-hidden="true"></span>
            </a>
        </div>
    }
}

#[derive(PartialEq, Properties)]
struct NavBarItemsProps {
    expanded_class: Option<String>,
}

#[function_component(NavBarItems)]
fn nav_bar_items(props: &NavBarItemsProps) -> Html {
    let app_store = use_store::<AppStateStore>();
    let state = app_store.get_state();
    let button = match state.as_ref() {
        AppState::Guest | AppState::RequestedGuest(_) => {
            html! {
                <Link<Route> route={Route::Login} classes={classes!("button", "is-primary")}>
                    <strong>{"Log In"}</strong>
                </Link<Route>>
            }
        }
        AppState::Tutor(token) | AppState::Admin(token, _) => {
            html! {
                <LogOutButton token={token.clone()}/>
            }
        }
    };

    html! {
        <div id="navbarBasic" class={classes!("navbar-menu", &props.expanded_class)}>
            <div class="navbar-end">
                <div class="navbar-item is-hidden-touch">
                    <a class="button is-info" href="http://github.com/r0m4n27" target="_blank">
                        <i class="fab fa-github-alt is-size-4"/>
                    </a>
                </div>

                <div class="navbar-item">
                    {button}
                </div>
            </div>
    </div>
    }
}

#[derive(PartialEq, Properties)]
struct LogOutButtonProps {
    token: String,
}

#[function_component(LogOutButton)]
fn log_out_button(props: &LogOutButtonProps) -> Html {
    let app_store = use_store::<AppStateStore>();
    let err_store = use_store::<IndexErrorStateStore>();

    let on_logout = {
        let token = props.token.clone();

        Callback::once(|_| {
            spawn_local(async {
                if let Err(err) = log_out_and_update(token, err_store, app_store).await {
                    log_1(&err.to_string().into())
                }
            })
        })
    };

    html! {
        <button class="button is-danger" onclick={on_logout}>
            {"Log Out"}
        </button>
    }
}

async fn log_out_and_update(
    token: String,
    err_store: StoreRef<IndexErrorStateStore>,
    app_store: StoreRef<AppStateStore>,
) -> Result<()> {
    match log_out(&token).await? {
        ApiResult::Err(err) => err_store
            .dispatch()
            .reduce(|state| *state = IndexErrorState(Some(err.message))),
        ApiResult::Ok(_) => app_store
            .dispatch()
            .reduce(|state| *state = AppState::Guest),
    }

    Ok(())
}
