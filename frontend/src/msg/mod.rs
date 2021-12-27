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
            Msg::UrlChanged(subs::UrlChanged(url)) => {
                model.current_url = url.clone();
                model.user.change_page(url)
            }
            Msg::Api(msg) => msg.update(model, orders),
            Msg::Page(msg) => msg.update(model, orders),
            Msg::RedirectIfNotFound => {
                if model.user.page().is_not_found() {
                    model.goto_index()
                } else {
                    orders.skip();
                }
            }
            Msg::Refresh => {
                match &model.user {
                    User::RequestedGuest(data) => {
                        orders.send_msg(Msg::Api(ApiMsg::Request(
                            RequestApiMsg::RefreshRequestedGuest(data.task.id.clone()),
                        )));
                    }
                    User::Admin(data) => {
                        orders.send_msg(Msg::Api(ApiMsg::Request(RequestApiMsg::RefreshAdmin(
                            data.token.clone(),
                        ))));
                    }
                    User::Tutor(data) => {
                        orders.send_msg(Msg::Api(ApiMsg::Request(RequestApiMsg::RefreshTutor(
                            data.token.clone(),
                        ))));
                    }
                    _ => {}
                }
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
