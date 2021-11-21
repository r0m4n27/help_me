use yew::prelude::*;
use yew_router::{Routable, Router};

use pages::{Index, Login, Register};

mod api;
mod components;
mod pages;
mod state;

#[derive(Debug, Routable, PartialEq, Clone)]
enum Route {
    #[at("/register")]
    Register,
    #[at("/login")]
    Login,
    #[at("/")]
    Index,
}

#[function_component(App)]
fn app() -> Html {
    let switch = |route: &Route| match route {
        Route::Index => {
            html! {<Index/>}
        }
        Route::Login => {
            html! {<Login/>}
        }
        Route::Register => {
            html! {<Register/>}
        }
    };

    html! {
        <Router<Route> render={Router::render(switch)} />
    }
}

fn main() {
    yew::start_app::<App>();
}
