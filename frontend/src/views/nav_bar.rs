use std::borrow::Cow;

use seed::prelude::*;

use crate::{
    model::{page::Urls, user::User, Model},
    msg::Msg,
};

pub fn nav_bar_view(model: &Model) -> Node<Msg> {
    nav![
        C!["navbar"],
        attrs! {
            At::Custom(Cow::Borrowed("role")) => "navigation",
            At::AriaLabel => "main navigation"
        },
        nav_bar_brand(model),
        nav_bar_items(model)
    ]
}

fn nav_bar_brand(model: &Model) -> Node<Msg> {
    let title = a![
        C!["navbar-item"],
        attrs! {
            At::Href => model.urls.index()
        },
        p![C!["title", "is-unselectable"], "Help Me"]
    ];

    let github_link = a![
        C!["has-text-light", "is-flex", "is-align-items-center"],
        attrs! {
            At::Href => "http://github.com/r0m4n27",
            At::Target => "_blank"
        },
        i![C!["fab", "fa-github-alt", "is-size-4"]]
    ];

    let burger = a![
        C!["navbar-burger", IF!(model.expanded_menu => "is-active")],
        attrs!(
            At::AriaLabel => "menu",
            At::AriaExpanded => "false",
            At::Custom(Cow::Borrowed("data-target")) => "navbarBasic"
        ),
        ev(Ev::Click, |_| Msg::ChangeMenu),
        span!(attrs!(At::AriaHidden => "true")),
        span!(attrs!(At::AriaHidden => "true")),
        span!(attrs!(At::AriaHidden => "true"))
    ];

    div![
        C!["navbar-brand"],
        title,
        div![C!["navbar-item", "is-hidden-desktop"], github_link, burger]
    ]
}

fn nav_bar_items(model: &Model) -> Node<Msg> {
    // TODO: Log Out
    // a![C!["button", "is-danger"], "Log Out"],
    let button = match model.user {
        User::Guest(_) | User::RequestedGuest(_, _) => a![
            C!["button", "is-primary"],
            "Log In",
            attrs! {
                At::Href => model.urls.login()
            }
        ],
    };

    let github = div![
        C!["navbar-item", "is-hidden-touch"],
        a![
            C!["button", "is-info"],
            attrs! {
                At::Href => "http://github.com/r0m4n27",
                At::Target => "_blank"
            },
            i![C!["fab", "fa-github-alt", "is-size-4"]]
        ]
    ];

    div![
        C!["navbar-menu", IF!(model.expanded_menu => "is-active")],
        attrs! {
            At::Id => "navbarBasic",
        },
        div![C!["navbar-end"], github, div![C!["navbar-item"], button]]
    ]
}
