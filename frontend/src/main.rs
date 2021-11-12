use yew::prelude::*;
use yew_router::{Routable, Router};

use pages::Index;

mod api;
mod components;
mod pages;
mod state;

#[derive(Debug, Routable, PartialEq, Clone)]
enum Route {
    #[at("/login")]
    Login,
    #[at("/")]
    Index,
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <Router<Route> render={Router::render(switch)}/>
    }
}

fn switch(route: &Route) -> Html {
    match route {
        Route::Index => {
            html! {
                <Index/>
            }
        }
        Route::Login => {
            html! {}
        }
    }
}

fn main() {
    yew::start_app::<App>();
}
