use seed::prelude::*;

use crate::{
    model::{page::tutor::TutorPage, user::TutorData, Model},
    msg::Msg,
    views::tasks_list_view,
};

use super::hero_view;

pub fn tutor_view(data: &TutorData, model: &Model) -> Node<Msg> {
    match &data.page {
        TutorPage::Index { .. } => hero_view(tasks_list_view(&data.tasks), model),
        TutorPage::NotFound => hero_view(div![], model),
    }
}
