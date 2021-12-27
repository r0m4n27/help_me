use seed::prelude::*;

use crate::{
    model::{page::tutor::TutorPage, user::TutorData, Model},
    msg::Msg,
    views::{tasks_list_view, tutor_task_view},
};

use super::hero_view;

pub fn tutor_view(data: &TutorData, model: &Model) -> Node<Msg> {
    match &data.page {
        TutorPage::Index { .. } => hero_view(tasks_list_view(&data.tasks, &model.urls), model),
        TutorPage::NotFound => hero_view(div![], model),
        // The task page is only constructed if the task id exists
        // so we shoud be able to safely unwrap it
        TutorPage::Task { task_id, .. } => {
            hero_view(tutor_task_view(data.tasks.get(task_id).unwrap()), model)
        }
    }
}
