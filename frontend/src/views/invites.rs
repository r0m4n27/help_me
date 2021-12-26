use std::collections::{BinaryHeap, HashSet};

use seed::prelude::*;

use crate::{
    api::admin::Invite,
    msg::{page::PageMsg, Msg},
};

pub fn invites_view(invites: &HashSet<Invite>) -> Node<Msg> {
    let sorted_invites: BinaryHeap<_> = invites.clone().into_iter().collect();
    let entries = sorted_invites
        .into_sorted_vec()
        .into_iter()
        .map(invite_view);

    div![
        div![
            C!["level", "is-mobile"],
            div![
                C!["level-left"],
                div![
                    C!["level-item"],
                    p![C!["title", "is-4", "has-text-dark"], "Invite Codes"]
                ],
                a![
                    C!["button", "is-primary", "level-item", "has-text-centered"],
                    "Generate Code",
                    ev(Ev::Click, |_| Msg::Page(PageMsg::CreateInvite))
                ]
            ]
        ],
        table![C!["table"], tbody![entries]]
    ]
}

fn invite_view(invite: Invite) -> Node<Msg> {
    tr![
        th![&invite.invite_code],
        td![a![
            C!["has-text-link", "is-unselectable"],
            "Delete",
            ev(Ev::Click, move |_| Msg::Page(PageMsg::DeleteInvite(invite)))
        ]]
    ]
}
