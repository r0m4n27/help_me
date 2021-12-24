use seed::prelude::*;

use serde::{Deserialize, Serialize};

use crate::Msg;

pub struct Model {
    pub expanded_menu: bool,
    pub user: User,
}

impl Model {
    pub fn init(url: Url, orders: &mut impl Orders<Msg>) -> Self {
        Model {
            expanded_menu: false,
            user: User::init(url, orders),
        }
    }

    pub fn change_menu(&mut self) {
        self.expanded_menu = !self.expanded_menu
    }

    pub fn save(&self) {
        self.user.save()
    }
}

#[derive(Serialize, Deserialize)]
pub enum User {
    Guest,
    RequestedGuest(String),
    Admin(String),
    Tutor(String),
}

impl User {
    pub fn init(_: Url, _: &mut impl Orders<Msg>) -> Self {
        LocalStorage::get(User::storage_key()).unwrap_or(User::Guest)
    }

    pub fn save(&self) {
        // Webstorage error can't be converted to an anyhow error
        LocalStorage::insert(User::storage_key(), self).expect("Can't save User")
    }

    #[inline]
    fn storage_key() -> &'static str {
        "USER"
    }
}
