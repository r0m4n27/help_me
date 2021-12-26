use seed::prelude::*;
use serde::{Deserialize, Serialize};

use crate::api::task::Task;

use super::page::{
    guest::GuestPage,
    requested_guest::{RequestedGuestIndexData, RequestedGuestPage},
    Page,
};

pub enum User {
    Guest(GuestData),
    RequestedGuest(RequestedGuestData),
}

pub struct GuestData(pub GuestPage);
pub struct RequestedGuestData {
    pub task: Task,
    pub page: RequestedGuestPage,
}

impl User {
    pub fn init(url: Url) -> Self {
        let saved_user = SavedUser::init();

        match saved_user {
            SavedUser::Guest => User::Guest(GuestData(url.into())),
            SavedUser::RequestedGuest(task) => User::RequestedGuest(RequestedGuestData {
                task,
                page: url.into(),
            }),
        }
    }

    pub fn save(&self) {
        SavedUser::save(self)
    }

    pub fn change_page(&mut self, url: Url) {
        match self {
            User::Guest(data) => data.0 = url.into(),
            User::RequestedGuest(data) => data.page = url.into(),
        }
    }

    pub fn page(&self) -> &dyn Page {
        match self {
            User::Guest(data) => &data.0,
            User::RequestedGuest(data) => &data.page,
        }
    }

    pub fn page_mut(&mut self) -> &mut dyn Page {
        match self {
            User::Guest(data) => &mut data.0,
            User::RequestedGuest(data) => &mut data.page,
        }
    }

    pub fn as_guest<F: FnOnce(&mut GuestData)>(&mut self, func: F) {
        if let User::Guest(data) = self {
            func(data)
        }
    }
    pub fn as_requested_guest<F: FnOnce(&mut RequestedGuestData)>(&mut self, func: F) {
        if let User::RequestedGuest(data) = self {
            func(data)
        }
    }
}

impl RequestedGuestData {
    pub fn start_editing(&mut self) {
        if let RequestedGuestPage::Index(data) = &mut self.page {
            *data = RequestedGuestIndexData::Editing {
                title_ref: ElRef::new(),
                description_ref: ElRef::new(),
                error: None,
            }
        }
    }

    pub fn cancel_editing(&mut self) {
        if let RequestedGuestPage::Index(data) = &mut self.page {
            *data = RequestedGuestIndexData::Viewing { error: None }
        }
    }

    pub fn update_task(&mut self, new_task: Task) {
        self.task = new_task;

        if let RequestedGuestPage::Index(data) = &mut self.page {
            *data = RequestedGuestIndexData::Viewing { error: None }
        }
    }
}

// This enum is used to store the relevant data
// of the user in the local storage
// It is used to reconstruct the user
#[derive(Serialize, Deserialize)]
enum SavedUser {
    Guest,
    RequestedGuest(Task),
}

impl SavedUser {
    fn init() -> Self {
        LocalStorage::get(Self::storage_key()).unwrap_or(SavedUser::Guest)
    }

    fn save(user: &User) {
        let saved_user = match user {
            User::Guest(_) => SavedUser::Guest,
            User::RequestedGuest(data) => SavedUser::RequestedGuest(data.task.clone()),
        };

        LocalStorage::insert(Self::storage_key(), &saved_user).expect("Can't save User")
    }

    #[inline]
    const fn storage_key() -> &'static str {
        "USER"
    }
}
