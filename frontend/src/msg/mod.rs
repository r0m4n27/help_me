use core::fmt;

use seed::prelude::*;

use self::{page::PageMsg, request::RequestApiMsg, response::ResponseApiMsg};
use crate::model::{user::User, Model};

pub mod page;
mod request;
mod response;

pub enum Msg {
    ChangeMenu,
    UrlChanged(subs::UrlChanged),
    RedirectIfNotFound,
    Refresh,
    Request(RequestApiMsg),
    Response(ResponseApiMsg),
    Page(PageMsg),
}

impl From<RequestApiMsg> for Msg {
    fn from(msg: RequestApiMsg) -> Self {
        Msg::Request(msg)
    }
}

impl Msg {
    pub fn update(self, model: &mut Model, orders: &mut impl Orders<Msg>) {
        match self {
            Msg::ChangeMenu => model.change_menu(),
            Msg::UrlChanged(subs::UrlChanged(url)) => {
                model.current_url = url.clone();
                model.user.change_page(url)
            }
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
                        orders.send_msg(
                            RequestApiMsg::RefreshRequestedGuest(data.task.id.clone()).into(),
                        );
                    }
                    User::Admin(data) => {
                        orders.send_msg(RequestApiMsg::RefreshAdmin(data.token.clone()).into());
                    }
                    User::Tutor(data) => {
                        orders.send_msg(RequestApiMsg::RefreshTutor(data.token.clone()).into());
                    }
                    _ => {}
                }
                orders.skip();
            }
            Msg::Request(msg) => {
                orders
                    .skip()
                    .perform_cmd(async move { msg.make_request().await });
            }
            Msg::Response(msg) => msg.update(model, orders),
        }

        model.save()
    }
}

fn log_err<T: fmt::Debug>(err: T) -> Option<Msg> {
    let message = format!("{:?}", err);
    log_1(&message.into());
    None
}
