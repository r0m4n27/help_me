use seed::prelude::*;

use crate::{
    api::task::Task,
    model::{page::RequestedGuestPages, Model},
    msg::Msg,
    views::guest_task_view,
};

use super::hero_view;

pub fn requested_guest_view(task: &Task, pages: &RequestedGuestPages, model: &Model) -> Node<Msg> {
    match pages {
        RequestedGuestPages::Index { page_data, .. } => {
            hero_view(guest_task_view(task, page_data), model)
        }
        RequestedGuestPages::NotFound => hero_view(div![], model),
    }
}
