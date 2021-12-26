use seed::prelude::*;

use crate::{
    model::page::{login::LoginPageData, Urls},
    msg::{page::PageMsg, Msg},
};

pub fn login_view(page_data: &LoginPageData, urls: &Urls) -> Node<Msg> {
    form![
        C!["box"],
        div![
            C!["content", "has-text-centered"],
            p![C!["title", "has-text-dark", "is-2"], "Login"]
        ],
        div![
            C!["content"],
            p![
                C!["title", "has-text-dark", "is-5", "level-left"],
                "User Name"
            ],
            input![
                C!["input"],
                attrs! {
                    At::Size => "50",
                    At::Type => "text",
                    At::AutoComplete => "username"
                },
                el_ref(&page_data.user_name_ref)
            ]
        ],
        div![
            C!["content"],
            p![
                C!["title", "has-text-dark", "is-5", "level-left"],
                "Password"
            ],
            input![
                C!["input"],
                attrs! {
                    At::Size => "50",
                    At::Type => "password",
                    At::AutoComplete => "current-password"
                },
                el_ref(&page_data.password_ref)
            ]
        ],
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
