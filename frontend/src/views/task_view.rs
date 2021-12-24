use seed::prelude::*;

use crate::msg::Msg;

pub struct TaskViewProps<'a> {
    pub header: &'a str,
    pub title: &'a str,
    pub description: &'a str,
    pub buttons: Node<Msg>,
}

pub fn task_view(props: TaskViewProps) -> Node<Msg> {
    div![
        C!["box"],
        div![
            C!["content", "has-text-centered"],
            p![C!["title", "has-text-dark", "is-2"], props.header]
        ],
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
