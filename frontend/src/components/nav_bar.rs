use yew::prelude::*;
use yew_router::components::Link;

use crate::Route;

#[derive(PartialEq, Properties)]
pub struct NavBarProps {
    pub logged_in: bool,
}

#[function_component(NavBar)]
pub fn nav_bar(props: &NavBarProps) -> Html {
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

            <NavBarItems expanded_class={expanded_class} logged_in={props.logged_in}/>
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
    logged_in: bool,
}

#[function_component(NavBarItems)]
fn nav_bar_items(props: &NavBarItemsProps) -> Html {
    let button = if props.logged_in {
        html! {
            <button class="button is-danger">
                {"Log Out"}
            </button>
        }
    } else {
        html! {
            <Link<Route> route={Route::Login} classes={classes!("button", "is-primary")}>
                <strong>{"Log In"}</strong>
            </Link<Route>>
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
