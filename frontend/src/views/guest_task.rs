use seed::prelude::{
    web_sys::{HtmlInputElement, HtmlTextAreaElement},
    *,
};

use crate::{
    api::task::Task, model::page::requested_guest::RequestedGuestIndexData, msg::page::PageMsg,
};

use super::{
    edit_task::{edit_task_view, EditTaskProps},
    task_view::{task_view, TaskViewProps},
};

pub fn guest_task_view(task: &Task, page_data: &RequestedGuestIndexData) -> Node<PageMsg> {
    match page_data {
        RequestedGuestIndexData::Viewing { .. } => guest_task_default_view(task),
        RequestedGuestIndexData::Editing {
            title_ref,
            description_ref,
            ..
        } => guest_task_edit_view(task, title_ref, description_ref),
    }
}

fn guest_task_default_view(task: &Task) -> Node<PageMsg> {
    let buttons = div![
        C!["buttons"],
        a![
            C!["button", "is-danger"],
            "Resolve",
            ev(Ev::Click, |_| PageMsg::ResolveTask)
        ],
        a![
            C!["button", "is-info"],
            "Edit",
            ev(Ev::Click, |_| PageMsg::EditTask)
        ]
    ];

    let how_many_ahead = format!("Queue: {}", task.how_many_ahead);
    let task_view_props = TaskViewProps {
        header: "Request Submitted",
        title: &task.title,
        description: &task.body,
        buttons,
        sub_header: Some(&how_many_ahead),
    };

    task_view(task_view_props)
}

fn guest_task_edit_view(
    task: &Task,
    title_ref: &ElRef<HtmlInputElement>,
    description_ref: &ElRef<HtmlTextAreaElement>,
) -> Node<PageMsg> {
    let buttons = div![
        C!["buttons"],
        a![
            C!["button", "is-danger"],
            "Cancel",
            ev(Ev::Click, |_| PageMsg::CancelEdit)
        ],
        a![
            C!["button", "is-primary"],
            "Confirm",
            ev(Ev::Click, |_| PageMsg::ConfirmEdit)
        ]
    ];

    let edit_task_props = EditTaskProps {
        header: "Edit Request",
        start_title: Some(&task.title),
        start_description: Some(&task.body),
        title_ref,
        description_ref,
        buttons,
    };

    edit_task_view(edit_task_props)
}
