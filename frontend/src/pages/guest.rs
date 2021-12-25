use seed::prelude::*;

use crate::{
    model::{page::GuestPages, Model},
    msg::Msg,
    views::submit_task_view,
};

use super::hero_view;

pub fn guest_view(pages: &GuestPages, model: &Model) -> Node<Msg> {
    match pages {
        GuestPages::Index {
            title_ref,
            description_ref,
            ..
        } => hero_view(submit_task_view(title_ref, description_ref), model),
        GuestPages::NotFound => hero_view(div![], model),
    }
}
