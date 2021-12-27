use seed::prelude::{
    web_sys::{HtmlInputElement, HtmlTextAreaElement},
    *,
};

use crate::views::util::{box_header_view, input_view};

pub struct EditTaskProps<'a, Msg> {
    pub header: &'a str,
    pub start_title: Option<&'a str>,
    pub start_description: Option<&'a str>,
    pub title_ref: &'a ElRef<HtmlInputElement>,
    pub description_ref: &'a ElRef<HtmlTextAreaElement>,
    pub buttons: Node<Msg>,
}

pub fn edit_task_view<Msg>(props: EditTaskProps<'_, Msg>) -> Node<Msg> {
    let title = props.start_title.unwrap_or("");
    let description = props.start_description.unwrap_or("");

    div![
        C!["box"],
        box_header_view(props.header),
        input_view(
            "Title",
            props.title_ref,
            attrs! {
                At::Value => title,
                At::Size => "50",
                At::Type => "text"
            }
        ),
        div![
            C!["content"],
            p![
                C!["title", "has-text-dark", "is-5", "level-left"],
                "Description"
            ],
            textarea![
                C!["textarea", "has-fixed-size"],
                el_ref(props.description_ref),
                attrs! {
                    At::Value => description,
                    At::Type => "textarea",
                    At::Size => "50"
                }
            ],
        ],
        props.buttons
    ]
}
