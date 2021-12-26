use std::collections::{BinaryHeap, HashSet};

use seed::prelude::*;

use crate::{
    api::user::ApiUser,
    msg::{page::PageMsg, Msg},
};

pub fn users_view(users: &HashSet<ApiUser>) -> Node<Msg> {
    let sorted_users: BinaryHeap<_> = users.clone().into_iter().collect();
    let entries = sorted_users.into_sorted_vec().into_iter().map(user_view);

    div![
        div![
            C!["level", "is-mobile"],
            div![
                C!["level-item"],
                p![C!["title", "is-4", "has-text-dark"], "Users"]
            ],
            // Use this so the Headline has the same position
            // as 'Invites'
            a![
                C![
                    "button",
                    "is-primary",
                    "level-item",
                    "has-text-centered",
                    "is-invisible"
                ],
                "Placeholder"
            ]
        ],
        table![C!["table"], tbody![entries]]
    ]
}

fn user_view(user: ApiUser) -> Node<Msg> {
    tr![
        th![&user.user_name],
        td![
            a![C!["has-text-link", "is-unselectable"], "Delete"],
            ev(Ev::Click, move |_| Msg::Page(PageMsg::DeleteUser(user)))
        ]
    ]
}
