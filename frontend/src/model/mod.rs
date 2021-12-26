use seed::prelude::*;

use self::{
    page::Urls,
    user::{GuestData, RequestedGuestData, User},
};
use crate::{api::task::Task, msg::Msg};

pub mod page;
pub mod user;

pub struct Model {
    pub expanded_menu: bool,
    pub user: User,
    pub urls: Urls,
}

#[derive(Clone)]
struct NotifyToken;

impl Model {
    pub fn init(url: Url, orders: &mut impl Orders<Msg>) -> Self {
        orders
            .subscribe(Msg::UrlChanged)
            .subscribe(|_: subs::UrlChanged| Msg::RedirectIfNotFound)
            .subscribe(|_: NotifyToken| Msg::RedirectIfNotFound)
            .notify(NotifyToken);

        Model {
            expanded_menu: false,
            user: User::init(url),
            urls: Urls::new(Url::new()),
        }
    }

    pub fn change_menu(&mut self) {
        self.expanded_menu = !self.expanded_menu
    }

    pub fn save(&self) {
        self.user.save()
    }

    pub fn switch_to_requested_user(&mut self, task: Task) {
        self.user = User::RequestedGuest(RequestedGuestData {
            task,
            page: self.urls.base_url.clone().into(),
        })
    }

    pub fn switch_to_guest(&mut self) {
        self.user = User::Guest(GuestData(self.urls.base_url.clone().into()))
    }

    pub fn switch_to_admin(&mut self, token: String) {
        self.user = User::Admin(token)
    }

    pub fn switch_to_tutor(&mut self, token: String) {
        self.user = User::Tutor(token)
    }
}
