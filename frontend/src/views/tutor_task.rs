use seed::prelude::*;

use crate::{
    api::task::Task,
    msg::{page::PageMsg, Msg},
};

use super::task_view::{task_view, TaskViewProps};

pub fn tutor_task_view(task: &Task) -> Node<Msg> {
    let cloned_task = task.clone();

    let buttons = match task.state.as_str() {
        "pending" => a![
            C!["button", "is-info"],
            "Start",
            ev(Ev::Click, move |_| Msg::Page(PageMsg::Process(cloned_task)))
        ],
        "doing" => a![
            C!["button", "is-primary"],
            "Finish",
            ev(Ev::Click, move |_| Msg::Page(PageMsg::Finish(cloned_task)))
        ],
        _ => div![],
    };

    let task_view_props = TaskViewProps {
        header: "Process Task",
        title: &task.title,
        description: &task.body,
        buttons,
        sub_header: None,
    };

    task_view(task_view_props)
}
