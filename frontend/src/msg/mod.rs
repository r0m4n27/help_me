use core::fmt;

use seed::prelude::*;

use self::{api::ApiMsg, page::PageMsg};
use crate::model::Model;

pub mod api;
pub mod page;

pub enum Msg {
    ChangeMenu,
    UrlChanged(subs::UrlChanged),
    RedirectIfNotFound,
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
                let should_redirect = model
                    .user
                    .redirect_if_not_found(model.urls.base_url.clone());

                if should_redirect {
                    model.urls.goto_index();
                } else {
                    orders.skip();
                }
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
