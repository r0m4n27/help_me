use std::collections::HashSet;

use seed::prelude::*;

use self::{
    page::Urls,
    user::{AdminData, GuestData, RequestedGuestData, User},
};
use crate::{
    api::{admin::Invite, task::Task, user::ApiUser},
    msg::Msg,
};

pub mod page;
pub mod user;

pub struct Model {
    pub expanded_menu: bool,
    pub user: User,
    pub urls: Urls,
}

#[derive(Clone)]
struct ChangeUrlToken;

#[derive(Clone)]
pub struct RefreshToken;

impl Model {
    pub fn init(url: Url, orders: &mut impl Orders<Msg>) -> Self {
        orders
            .subscribe(Msg::UrlChanged)
            .subscribe(|_: subs::UrlChanged| Msg::RedirectIfNotFound)
            .subscribe(|_: ChangeUrlToken| Msg::RedirectIfNotFound)
            .subscribe(|_: RefreshToken| Msg::Refresh)
            .stream(streams::interval(30_000, || Msg::Refresh))
            .notify(RefreshToken)
            .notify(ChangeUrlToken);

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

    pub fn switch_to_admin(
        &mut self,
        token: String,
        invites: HashSet<Invite>,
        users: HashSet<ApiUser>,
    ) {
        self.user = User::Admin(AdminData {
            token,
            invites,
            users,
            page: self.urls.base_url.clone().into(),
        })
    }

    pub fn switch_to_tutor(&mut self, token: String) {
        self.user = User::Tutor(token)
    }
}
