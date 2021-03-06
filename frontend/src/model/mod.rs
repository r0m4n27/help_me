use std::collections::{HashMap, HashSet};

use seed::prelude::*;

use self::{
    page::{tutor::TutorPage, Urls},
    user::{AdminData, GuestData, RequestedGuestData, TutorData, User},
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
    pub current_url: Url,
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
            .stream(streams::interval(10_000, || Msg::Refresh))
            .notify(RefreshToken)
            .notify(ChangeUrlToken);

        Model {
            expanded_menu: false,
            user: User::init(url.clone()),
            current_url: url,
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
            page: self.current_url.clone().into(),
        })
    }

    pub fn switch_to_guest(&mut self) {
        self.user = User::Guest(GuestData(self.current_url.clone().into()))
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
            page: self.current_url.clone().into(),
        })
    }

    pub fn switch_to_tutor(&mut self, token: String, tasks: HashMap<String, Task>) {
        let page = TutorPage::new(self.current_url.clone(), &tasks);

        self.user = User::Tutor(TutorData { token, tasks, page })
    }

    pub fn goto_index(&mut self) {
        self.urls.goto_index();
        self.current_url = self.urls.base_url.clone();
        self.user.change_page(self.current_url.clone())
    }
}
