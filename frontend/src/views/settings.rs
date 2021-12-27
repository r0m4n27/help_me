use seed::prelude::*;

use crate::{
    model::page::settings::SettingsPageData,
    msg::page::PageMsg,
    views::util::{box_header_view, input_view},
};

pub fn settings_view(page_data: &SettingsPageData) -> Node<PageMsg> {
    form![
        C!["box"],
        box_header_view("Settings"),
        div![
            C!["columns"],
            div![
                C!["column"],
                input_view(
                    "Change User Name",
                    &page_data.user_name_ref,
                    attrs! {
                        At::Size => "50",
                        At::Type => "text",
                        At::AutoComplete => "username"
                    }
                ),
                input_view(
                    "Retype User Name",
                    &page_data.user_name_again_ref,
                    attrs! {
                        At::Size => "50",
                        At::Type => "text",
                        At::AutoComplete => "username"
                    }
                ),
                a![
                    C!["button", "is-primary"],
                    "Change User Name",
                    ev(Ev::Click, |_| PageMsg::ChangeUsername)
                ],
            ],
            div![
                C!["column"],
                input_view(
                    "Change Password",
                    &page_data.password_ref,
                    attrs! {
                        At::Size => "50",
                        At::Type => "password",
                        At::AutoComplete => "current-password"
                    }
                ),
                input_view(
                    "Retype Password",
                    &page_data.password_again_ref,
                    attrs! {
                        At::Size => "50",
                        At::Type => "password"
                    }
                ),
                a![
                    C!["button", "is-primary"],
                    "Change Password",
                    ev(Ev::Click, |_| PageMsg::ChangePassword)
                ],
            ]
        ]
    ]
}
