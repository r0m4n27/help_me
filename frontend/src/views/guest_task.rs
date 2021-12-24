use seed::prelude::*;

use crate::{api::task::Task, msg::Msg};

use super::task_view::{task_view, TaskViewProps};

pub fn guest_task_view(task: &Task) -> Node<Msg> {
    guest_task_default_view(task)
}

fn guest_task_default_view(task: &Task) -> Node<Msg> {
    // TODO: Interaction
    let buttons = div![
        C!["buttons"],
        a![C!["button", "is-danger"], "Resolve"],
        a![C!["button", "is-info"], "Edit"]
    ];

    let task_view_props = TaskViewProps {
        header: "Request Submitted",
        title: &task.title,
        description: &task.body,
        buttons,
    };

    task_view(task_view_props)
}
