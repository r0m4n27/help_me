use seed::prelude::*;

use crate::{
    api::task::Task,
    model::{page::requested_guest::RequestedGuestPage, Model},
    msg::Msg,
    views::{guest_task_view, login_view},
};

use super::hero_view;

pub fn requested_guest_view(task: &Task, pages: &RequestedGuestPage, model: &Model) -> Node<Msg> {
    match pages {
        RequestedGuestPage::Index(data) => hero_view(guest_task_view(task, data), model),
        RequestedGuestPage::NotFound => hero_view(div![], model),
        RequestedGuestPage::Login(page_data) => {
            hero_view(login_view(page_data, &model.urls), model)
        }
    }
}
