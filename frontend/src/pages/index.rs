use seed::prelude::*;

use crate::{model::Model, msg::Msg, views::nav_bar};

pub fn index_view(model: &Model) -> Node<Msg> {
    index_guest_view(model)
}

fn index_guest_view(model: &Model) -> Node<Msg> {
    section![
        C!["hero", "is-info", "is-fullheight"],
        div![C!["hero-head"], nav_bar(model)],
        div![C!["hero-body", "section"], div![C!["container"]]]
    ]
}
