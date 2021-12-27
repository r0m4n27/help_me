use seed::prelude::*;

use crate::{
    model::{user::User, Model},
    msg::Msg,
    views::nav_bar_view,
};

use self::{
    admin::admin_view, guest::guest_view, requested_guest::requested_guest_view, tutor::tutor_view,
};

mod admin;
mod guest;
mod requested_guest;
mod tutor;

pub fn page_view(model: &Model) -> Node<Msg> {
    match &model.user {
        User::Guest(data) => guest_view(&data.0, model),
        User::RequestedGuest(data) => requested_guest_view(&data.task, &data.page, model),
        User::Admin(data) => admin_view(data, model),
        User::Tutor(data) => tutor_view(data, model),
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
    match model.user.page().error_message() {
        Some(err) => div![C!["notification", "is-danger"], err],
        None => div![],
    }
}
