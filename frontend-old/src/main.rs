use yew::prelude::*;
use yew_router::{BrowserRouter, Routable, Switch};

use pages::{Index, Login, Register, Task};

mod api;
mod components;
mod pages;
mod state;

#[derive(Debug, Routable, PartialEq, Clone)]
enum Route {
    #[at("/task/:task_id")]
    Task { task_id: String },
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
        Route::Task { task_id } => {
            html! {<Task task_id={task_id.clone()}/>}
        }
    };

    html! {
        <BrowserRouter>
            <Switch<Route> render={Switch::render(switch)}/>
        </BrowserRouter>
    }
}

fn main() {
    yew::start_app::<App>();
}
