use seed::prelude::*;
use serde::{Deserialize, Serialize};

use crate::api::task::Task;

use super::page::{GuestPages, Page, RequestedGuestPages};

pub enum User {
    Guest(GuestPages),
    RequestedGuest(Task, RequestedGuestPages),
}

// This enum is used to store the relevant data
// of the user in the local storage
// It is used to reconstruct the user
#[derive(Serialize, Deserialize)]
enum SavedUser {
    Guest,
    RequestedGuest(Task),
}

impl User {
    pub fn init(url: Url) -> Self {
        let saved_user = LocalStorage::get(User::storage_key()).unwrap_or(SavedUser::Guest);

        match saved_user {
            SavedUser::Guest => User::Guest(GuestPages::new(url)),
            SavedUser::RequestedGuest(task) => {
                let pages = RequestedGuestPages::new(url, &task);
                User::RequestedGuest(task, pages)
            }
        }
    }

    pub fn save(&self) {
        let saved_user = match self {
            User::Guest(_) => SavedUser::Guest,
            User::RequestedGuest(task, _) => SavedUser::RequestedGuest(task.clone()),
        };

        // Webstorage error can't be converted to an anyhow error
        LocalStorage::insert(User::storage_key(), &saved_user).expect("Can't save User")
    }

    pub fn change_page(&mut self, url: Url) {
        match self {
            User::Guest(old_pages) => *old_pages = GuestPages::new(url),
            User::RequestedGuest(task, old_pages) => {
                *old_pages = RequestedGuestPages::new(url, task)
            }
        }
    }

    pub fn update_error(&mut self, error: String) {
        match self {
            User::Guest(page) => page.set_error_message(error),
            User::RequestedGuest(_, page) => page.set_error_message(error),
        }
    }

    pub fn error_message(&self) -> Option<&String> {
        match self {
            User::Guest(page) => page.error_message(),
            User::RequestedGuest(_, page) => page.error_message(),
        }
    }

    pub fn redirect_if_not_found(&mut self, url: Url) -> bool {
        let should_redirect = match self {
            User::Guest(pages) => pages.not_found(),
            User::RequestedGuest(_, pages) => pages.not_found(),
        };

        if should_redirect {
            self.change_page(url)
        }

        should_redirect
    }

    #[inline]
    fn storage_key() -> &'static str {
        "USER"
    }
}
