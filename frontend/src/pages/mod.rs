use seed::prelude::*;

use crate::{
    model::{user::User, Model},
    msg::Msg,
    views::nav_bar_view,
};

use self::{guest::guest_view, requested_guest::requested_guest_view};

mod guest;
mod requested_guest;

pub fn page_view(model: &Model) -> Node<Msg> {
    match &model.user {
        User::Guest(pages) => guest_view(pages, model),
        User::RequestedGuest(task, pages) => requested_guest_view(task, pages, model),
    }
}

fn hero_view(content: Node<Msg>, model: &Model) -> Node<Msg> {
    section![
        C!["hero", "is-info", "is-fullheight"],
        div![C!["hero-head"], nav_bar_view(model)],
        div![
            C!["hero-body", "section"],
            div![C!["container"], content, error_message_view(model)]
        ]
    ]
}

fn error_message_view(model: &Model) -> Node<Msg> {
    match model.user.error_message() {
        Some(err) => div![C!["notification", "is-danger"], err],
        None => div![],
    }
}
