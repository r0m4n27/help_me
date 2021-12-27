use seed::prelude::*;

use crate::{
    model::page::{login::LoginPageData, Urls},
    msg::{page::PageMsg, Msg},
    views::util::{box_header_view, input_view},
};

pub fn login_view(page_data: &LoginPageData, urls: &Urls) -> Node<Msg> {
    form![
        C!["box"],
        box_header_view("Login"),
        input_view(
            "User Name",
            &page_data.user_name_ref,
            attrs! {
                At::Size => "50",
                At::Type => "text",
                At::AutoComplete => "username"
            }
        ),
        input_view(
            "Password",
            &page_data.password_ref,
            attrs! {
                At::Size => "50",
                At::Type => "password",
                At::AutoComplete => "current-password"
            }
        ),
        div![
            C!["level", "content"],
            div![
                C!["level-left"],
                a![
                    C!["button", "is-primary", "level-item"],
                    "Login",
                    ev(Ev::Click, |_| Msg::Page(PageMsg::Login))
                ],
                a![
                    C!["has-text-link", "level-item"],
                    attrs! {
                        At::Href => urls.register()
                    },
                    "No account? Register with an invite code"
                ]
            ],
        ]
    ]
}
