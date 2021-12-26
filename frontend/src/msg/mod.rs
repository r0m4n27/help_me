use core::fmt;

use seed::prelude::*;

use self::{
    api::{ApiMsg, RequestApiMsg},
    page::PageMsg,
};
use crate::model::{user::User, Model};

pub mod api;
pub mod page;

pub enum Msg {
    ChangeMenu,
    UrlChanged(subs::UrlChanged),
    RedirectIfNotFound,
    Refresh,
    Api(ApiMsg),
    Page(PageMsg),
}

impl Msg {
    pub fn update(self, model: &mut Model, orders: &mut impl Orders<Msg>) {
        match self {
            Msg::ChangeMenu => model.change_menu(),
            Msg::UrlChanged(subs::UrlChanged(url)) => model.user.change_page(url),
            Msg::Api(msg) => msg.update(model, orders),
            Msg::Page(msg) => msg.update(model, orders),
            Msg::RedirectIfNotFound => {
                if model.user.page().is_not_found() {
                    model.urls.goto_index();
                    model.user.change_page(model.urls.base_url.clone())
                } else {
                    orders.skip();
                }
            }
            Msg::Refresh => {
                if let User::RequestedGuest(data) = &model.user {
                    orders.send_msg(Msg::Api(ApiMsg::Request(
                        RequestApiMsg::RefreshRequestedGuest(data.task.id.clone()),
                    )));
                };
                orders.skip();
            }
        }

        model.save()
    }
}

fn log_err<T: fmt::Debug>(err: T) -> Option<Msg> {
    let message = format!("{:?}", err);
    log_1(&message.into());
    None
}
