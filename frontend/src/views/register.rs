use seed::prelude::*;

use crate::{
    model::page::register::RegisterPageData,
    msg::{page::PageMsg, Msg},
    views::util::{box_header_view, input_view},
};

pub fn register_view(page_data: &RegisterPageData) -> Node<Msg> {
    form![
        C!["box"],
        box_header_view("Register"),
        input_view(
            "User Name",
            &page_data.user_name_ref,
            attrs! {
                At::Type => "text",
                At::Size => "50",
                At::AutoComplete => "username"
            }
        ),
        input_view(
            "Password",
            &page_data.password_ref,
            attrs! {
                At::Type => "password",
                At::Size => "50",
                At::AutoComplete => "current-password"
            }
        ),
        input_view(
            "Invite Code",
            &page_data.invite_code_ref,
            attrs! {
                At::Type => "text",
                At::Size => "50"
            }
        ),
        a![
            C!["button", "is-primary"],
            "Register",
            ev(Ev::Click, |_| Msg::Page(PageMsg::Register))
        ]
    ]
}
