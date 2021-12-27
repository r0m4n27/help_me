use seed::prelude::*;

use crate::{
    model::{page::guest::GuestPage, Model},
    msg::Msg,
    views::{login_view, register_view, submit_task_view},
};

use super::hero_view;

pub fn guest_view(pages: &GuestPage, model: &Model) -> Node<Msg> {
    match pages {
        GuestPage::Index(data) => hero_view(
            submit_task_view(&data.title_ref, &data.description_ref).map_msg(Msg::Page),
            model,
        ),
        GuestPage::NotFound => hero_view(div![], model),
        GuestPage::Login(page_data) => {
            hero_view(login_view(page_data, &model.urls).map_msg(Msg::Page), model)
        }
        GuestPage::Register(data) => hero_view(register_view(data).map_msg(Msg::Page), model),
    }
}
