use std::borrow::Cow;

use seed::prelude::*;

use crate::{
    model::{Model, Urls, User},
    msg::Msg,
};

pub fn nav_bar(model: &Model) -> Node<Msg> {
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
    let urls = Urls::new(&model.base_url);
    let title = a![
        C!["navbar-item"],
        attrs! {
            At::Href => urls.index()
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
    let urls = Urls::new(&model.base_url);

    let button = match model.user {
        User::Guest | User::RequestedGuest(_) => a![
            C!["button", "is-primary"],
            "Log In",
            attrs! {
                At::Href => urls.login()
            }
        ],
        User::Admin(_) | User::Tutor(_) => a![C!["button", "is-danger"], "Log Out"],
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
