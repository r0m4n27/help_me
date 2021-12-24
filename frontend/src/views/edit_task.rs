use seed::prelude::{
    web_sys::{HtmlInputElement, HtmlTextAreaElement},
    *,
};

use crate::msg::Msg;

pub struct EditTaskProps<'a> {
    pub header: &'a str,
    pub start_title: Option<&'a str>,
    pub start_description: Option<&'a str>,
    pub title_ref: ElRef<HtmlInputElement>,
    pub description_ref: ElRef<HtmlTextAreaElement>,
    pub buttons: Node<Msg>,
}

pub fn edit_task_view(props: EditTaskProps<'_>) -> Node<Msg> {
    let title = props.start_title.unwrap_or("");
    let description = props.start_description.unwrap_or("");

    div![
        C!["box"],
        div![
            C!["content", "has-text-centered"],
            p![C!["title", "has-text-dark", "is-2"], props.header],
        ],
        div![
            C!["content"],
            p![C!["title", "has-text-dark", "is-5", "level-left"], "Title"],
            input![
                C!["input"],
                el_ref(&props.title_ref),
                attrs! {
                    At::Value => title,
                    At::Size => "50",
                    At::Type => "text"
                }
            ]
        ],
        div![
            C!["content"],
            p![
                C!["title", "has-text-dark", "is-5", "level-left"],
                "Description"
            ],
            textarea![
                C!["textarea", "has-fixed-size"],
                el_ref(&props.description_ref),
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
