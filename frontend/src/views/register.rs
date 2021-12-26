use seed::prelude::*;

use crate::{
    model::page::register::RegisterPageData,
    msg::{page::PageMsg, Msg},
};

pub fn register_view(page_data: &RegisterPageData) -> Node<Msg> {
    form![
        C!["box"],
        div![
            C!["content", "has-text-centered"],
            p![C!["title", "has-text-dark", "is-2"], "Register"]
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
                    At::Type => "text",
                    At::Size => "50",
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
                    At::Type => "password",
                    At::Size => "50",
                    At::AutoComplete => "current-password"
                },
                el_ref(&page_data.password_ref)
            ]
        ],
        div![
            C!["content"],
            p![
                C!["title", "has-text-dark", "is-5", "level-left"],
                "Invite Code"
            ],
            input![
                C!["input"],
                attrs! {
                    At::Type => "text",
                    At::Size => "50"
                },
                el_ref(&page_data.invite_code_ref)
            ]
        ],
        a![
            C!["button", "is-primary"],
            "Register",
            ev(Ev::Click, |_| Msg::Page(PageMsg::Register))
        ]
    ]
}
