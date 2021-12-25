use seed::prelude::{
    web_sys::{HtmlInputElement, HtmlTextAreaElement},
    *,
};

use crate::msg::{page::PageMsg, Msg};

use super::edit_task::{edit_task_view, EditTaskProps};

pub fn submit_task_view(
    title_ref: &ElRef<HtmlInputElement>,
    description_ref: &ElRef<HtmlTextAreaElement>,
) -> Node<Msg> {
    let buttons = div![button![
        C!["button", "is-primary"],
        "Submit",
        ev(Ev::Click, move |_| { Msg::Page(PageMsg::Submit) })
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
