use seed::prelude::*;

use crate::{
    model::page::settings::SettingsPageData,
    msg::{page::PageMsg, Msg},
};

pub fn settings_view(page_data: &SettingsPageData) -> Node<Msg> {
    form![
        C!["box"],
        div![
            C!["content", "has-text-centered"],
            p![C!["title", "has-text-dark", "is-2"], "Settings"]
        ],
        div![
            C!["columns"],
            div![
                C!["column"],
                div![
                    C!["content"],
                    p![
                        C!["title", "has-text-dark", "is-5", "level-left"],
                        "Change User Name"
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
                        "Retype User Name"
                    ],
                    input![
                        C!["input"],
                        attrs! {
                            At::Size => "50",
                            At::Type => "text",
                            At::AutoComplete => "username"
                        },
                        el_ref(&page_data.user_name_again_ref)
                    ]
                ],
                a![
                    C!["button", "is-primary"],
                    "Change User Name",
                    ev(Ev::Click, |_| Msg::Page(PageMsg::ChangeUsername))
                ],
            ],
            div![
                C!["column"],
                div![
                    C!["content"],
                    p![
                        C!["title", "has-text-dark", "is-5", "level-left"],
                        "Change Password"
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
                    C!["content"],
                    p![
                        C!["title", "has-text-dark", "is-5", "level-left"],
                        "Retype Password"
                    ],
                    input![
                        C!["input"],
                        attrs! {
                            At::Size => "50",
                            At::Type => "password",
                            At::AutoComplete => "current-password"
                        },
                        el_ref(&page_data.password_again_ref)
                    ]
                ],
                a![
                    C!["button", "is-primary"],
                    "Change Password",
                    ev(Ev::Click, |_| Msg::Page(PageMsg::ChangePassword))
                ],
            ]
        ]
    ]
}
