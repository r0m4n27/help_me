use seed::prelude::*;

use crate::views::util::box_header_view;

pub struct TaskViewProps<'a, Msg> {
    pub header: &'a str,
    pub sub_header: Option<&'a str>,
    pub title: &'a str,
    pub description: &'a str,
    pub buttons: Node<Msg>,
}

pub fn task_view<Msg>(props: TaskViewProps<'_, Msg>) -> Node<Msg> {
    let sub_header = if let Some(sub_header) = props.sub_header {
        div![
            C!["content", "has-text-centered"],
            p![C!["title", "has-text-dark", "is-4"], sub_header],
        ]
    } else {
        div![]
    };

    div![
        C!["box"],
        box_header_view(props.header),
        sub_header,
        div![
            C!["content"],
            p![
                C!["is-size-4", "has-text-weight-bold"],
                style! {
                    St::WordWrap => "break-word"
                },
                props.title
            ]
        ],
        div![
            C!["content"],
            p![
                C!["is-size-5"],
                style! {
                    St::WordWrap => "break-word"
                },
                props.description
            ]
        ],
        props.buttons
    ]
}
