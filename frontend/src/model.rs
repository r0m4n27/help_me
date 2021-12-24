use std::mem;

use seed::prelude::*;

use serde::{Deserialize, Serialize};

use crate::api::task::Task;

pub struct Model {
    pub expanded_menu: bool,
    pub user: User,
    pub page: Page,
    pub base_url: Url,
}

impl Model {
    pub fn init(url: Url) -> Self {
        Model {
            expanded_menu: false,
            user: User::init(),
            base_url: url.to_base_url(),
            page: Page::init(url),
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
    RequestedGuest(Task),
    Admin(String),
    Tutor(String),
}

impl User {
    fn init() -> Self {
        LocalStorage::get(User::storage_key()).unwrap_or(User::Guest)
    }

    fn save(&self) {
        // Webstorage error can't be converted to an anyhow error
        LocalStorage::insert(User::storage_key(), self).expect("Can't save User")
    }

    #[inline]
    fn storage_key() -> &'static str {
        "USER"
    }
}

pub enum Page {
    Index {
        error: Option<String>,
    },
    Login {
        error: Option<String>,
    },
    Register {
        error: Option<String>,
    },
    Task {
        task_id: String,
        error: Option<String>,
    },
}

const LOGIN_PART: &str = "login";
const REGISTER_PART: &str = "register";
const TASK_PART: &str = "task";

impl Page {
    pub fn init(mut url: Url) -> Self {
        match url.remaining_path_parts().as_slice() {
            [] => Page::Index { error: None },
            [LOGIN_PART] => Page::Login { error: None },
            [REGISTER_PART] => Page::Register { error: None },
            [TASK_PART, task_id] => Page::Task {
                error: None,
                task_id: task_id.to_string(),
            },
            _ => Page::Index { error: None },
        }
    }

    pub fn error(&self) -> &Option<String> {
        match self {
            Page::Index { error } => error,
            Page::Login { error } => error,
            Page::Register { error } => error,
            Page::Task { task_id: _, error } => error,
        }
    }

    pub fn update_error(&mut self, message: String) {
        let old_error = match self {
            Page::Index { error } => error,
            Page::Login { error } => error,
            Page::Register { error } => error,
            Page::Task { task_id: _, error } => error,
        };
        let mut new_error = Some(message);
        mem::swap(old_error, &mut new_error)
    }

    #[inline]
    pub const fn login_url() -> &'static str {
        LOGIN_PART
    }
    #[inline]
    pub const fn register_url() -> &'static str {
        REGISTER_PART
    }
    #[inline]
    pub const fn task_url() -> &'static str {
        TASK_PART
    }
}

struct_urls!();
impl<'a> Urls<'a> {
    pub fn index(self) -> Url {
        self.base_url()
    }

    pub fn login(self) -> Url {
        self.base_url().add_path_part(LOGIN_PART)
    }
}
