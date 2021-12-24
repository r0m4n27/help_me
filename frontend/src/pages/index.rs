use seed::prelude::*;

use crate::{
    model::Model,
    msg::Msg,
    views::{guest_task_view, submit_task_view},
};

use super::hero_view;

pub fn index_view(model: &Model) -> Node<Msg> {
    match &model.user {
        crate::model::User::Guest => hero_view(submit_task_view(), model),
        crate::model::User::RequestedGuest(task) => hero_view(guest_task_view(task), model),
        crate::model::User::Admin(_) => todo!(),
        crate::model::User::Tutor(_) => todo!(),
    }
}
