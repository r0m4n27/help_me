use std::collections::HashSet;

use seed::prelude::*;

use crate::{
    api::admin::Invite,
    model::{page::admin::AdminPage, Model},
    msg::Msg,
    views::invites_view,
};

use super::hero_view;

pub fn admin_view(invites: &HashSet<Invite>, page: &AdminPage, model: &Model) -> Node<Msg> {
    match page {
        AdminPage::Index => hero_view(
            div![
                C!["box"],
                div![C!["columns"], div![C!["column"], invites_view(invites)]]
            ],
            model,
        ),
        AdminPage::NotFound => hero_view(div![], model),
    }
}
