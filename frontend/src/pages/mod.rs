use yew::prelude::*;

mod index;
mod login;
mod register;

pub use index::Index;
pub use login::Login;
pub use register::Register;

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
