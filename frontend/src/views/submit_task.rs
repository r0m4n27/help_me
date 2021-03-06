use seed::prelude::{
    web_sys::{HtmlInputElement, HtmlTextAreaElement},
    *,
};

use crate::msg::page::PageMsg;

use super::edit_task::{edit_task_view, EditTaskProps};

pub fn submit_task_view(
    title_ref: &ElRef<HtmlInputElement>,
    description_ref: &ElRef<HtmlTextAreaElement>,
) -> Node<PageMsg> {
    let buttons = div![button![
        C!["button", "is-primary"],
        "Submit",
        ev(Ev::Click, |_| PageMsg::SubmitTask)
    ]];

    let edit_task_props = EditTaskProps {
        header: "Submit Request",
        start_title: None,
        start_description: None,
        title_ref,
        description_ref,
        buttons,
    };

    edit_task_view(edit_task_props)
}
