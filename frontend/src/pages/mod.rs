use seed::prelude::*;

use crate::{model::Model, msg::Msg, views::nav_bar_view};

use self::index::index_view;

mod index;

pub fn page_view(model: &Model) -> Node<Msg> {
    match &model.page {
        crate::model::Page::Index { error: _ } => index_view(model),
        crate::model::Page::Login { error: _ } => div![],
        crate::model::Page::Register { error: _ } => div![],
        crate::model::Page::Task {
            task_id: _,
            error: _,
        } => div![],
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
    match model.page.error() {
        Some(err) => div![C!["notification", "is-danger"], err],
        None => div![],
    }
}
