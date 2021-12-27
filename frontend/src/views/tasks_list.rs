use std::collections::{BinaryHeap, HashMap};

use seed::prelude::*;

use crate::{api::task::Task, model::page::Urls, msg::Msg};

pub fn tasks_list_view(tasks: &HashMap<String, Task>, urls: &Urls) -> Node<Msg> {
    let sorted_tasks: BinaryHeap<_> = tasks.iter().map(|(_, task)| task).collect();
    let elems = sorted_tasks
        .into_sorted_vec()
        .into_iter()
        .map(|task| task_view(task, urls));

    div![
        C!["box"],
        div![
            C!["content", "has-text-centered"],
            p![C!["title", "has-text-dark", "is-2"], "Requests"]
        ],
        div![
            C!["table-container", "columns"],
            table![C!["table", "column", "is-6", "is-offset-3"], tbody![elems]]
        ]
    ]
}

fn task_view(task: &Task, urls: &Urls) -> Node<Msg> {
    tr![
        th![
            C!["content"],
            p![C!["is-bold"], strong![task.state.to_uppercase()]]
        ],
        td![a![
            p![
                C!["has-text-link", "is-unselectable", "is-hidden-touch"],
                style! {
                    St::Display => "inline-block",
                    St::WhiteSpace => "nowrap",
                    St::Overflow => "hidden",
                    St::TextOverflow => "ellipsis",
                    St::MaxWidth => "80ch"
                },
                &task.title
            ],
            attrs! {
                At::Href => urls.task(&task.id)
            }
        ]],
        td![a![
            p![
                C!["has-text-link", "is-unselectable", "is-hidden-desktop"],
                style! {
                    St::Display => "inline-block",
                    St::WhiteSpace => "nowrap",
                    St::Overflow => "hidden",
                    St::TextOverflow => "ellipsis",
                    St::MaxWidth => "20ch"
                },
                &task.title
            ],
            attrs! {
                At::Href => urls.task(&task.id)
            }
        ]]
    ]
}
