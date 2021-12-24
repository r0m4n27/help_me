use seed::prelude::{
    web_sys::{HtmlInputElement, HtmlTextAreaElement},
    *,
};

use crate::msg::{Msg, RequestApiMsg};

use super::edit_task::{edit_task_view, EditTaskProps};

pub fn submit_task_view() -> Node<Msg> {
    let title_ref: ElRef<HtmlInputElement> = ElRef::new();
    let description_ref: ElRef<HtmlTextAreaElement> = ElRef::new();

    let buttons = {
        let title_ref = title_ref.clone();
        let description_ref = description_ref.clone();

        div![button![
            C!["button", "is-primary"],
            "Submit",
            ev(Ev::Click, move |_| {
                let title = title_ref.get().expect("Title isn't redered").value();
                let description = description_ref
                    .get()
                    .expect("Description isn't redered")
                    .value();

                Msg::RequestApi(RequestApiMsg::SubmitTask(title, description))
            })
        ]]
    };

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
