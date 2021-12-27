use seed::prelude::*;

use crate::{
    model::{page::admin::AdminPage, user::AdminData, Model},
    msg::Msg,
    views::{invites_view, settings_view, users_view},
};

use super::hero_view;

pub fn admin_view(data: &AdminData, model: &Model) -> Node<Msg> {
    match &data.page {
        AdminPage::Index { .. } => hero_view(
            div![
                C!["box"],
                div![
                    C!["columns"],
                    div![C!["column"], invites_view(&data.invites)],
                    div![C!["column"], users_view(&data.users)]
                ]
            ],
            model,
        ),
        AdminPage::NotFound => hero_view(div![], model),
        AdminPage::Settings(data) => hero_view(settings_view(data), model),
    }
}
