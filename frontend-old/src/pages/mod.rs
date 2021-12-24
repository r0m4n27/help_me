use wasm_bindgen::{prelude::Closure, JsCast};
use wasm_bindgen_futures::spawn_local;
use web_sys::window;
use yew::prelude::*;

use crate::components::NavBar;

mod index;
mod login;
mod register;
mod task;

pub use index::Index;
pub use login::Login;
pub use register::Register;
pub use task::{Task, TaskProps};

#[derive(PartialEq, Properties)]
struct ErrorMessageProps {
    err: Option<String>,
}

#[function_component(ErrorMessage)]
fn error_message(props: &ErrorMessageProps) -> Html {
    match props.err.as_ref() {
        Some(err) => html! {
            <div class="notification is-danger">
                <p>{err}</p>
            </div>
        },
        None => html! {},
    }
}

#[function_component(UninitialisedView)]
fn uninitialised_view() -> Html {
    html! {
        <section class="hero is-info is-fullheight">
            <div class="hero-head">
                <NavBar/>
            </div>

            <div class="hero-body section">
            </div>
        </section>
    }
}

fn on_init<F: FnOnce() + 'static + ?Sized>(fun: Closure<F>) {
    use_effect_with_deps(
        move |_| {
            spawn_local(async move {
                let window = window().unwrap();
                let closure = fun;
                window
                    .set_timeout_with_callback_and_timeout_and_arguments_0(
                        closure.as_ref().unchecked_ref(),
                        200,
                    )
                    .unwrap();
            });
            || {}
        },
        true,
    );
}
