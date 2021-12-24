use seed::prelude::*;

use serde::{Deserialize, Serialize};

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
    RequestedGuest(String),
    Admin(String),
    Tutor(String),
}

impl User {
    pub fn init() -> Self {
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
        match url.remaining_hash_path_parts().as_slice() {
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

    pub fn task(self, task_id: &str) -> Url {
        self.base_url()
            .add_hash_path_part(TASK_PART)
            .add_hash_path_part(task_id)
    }

    pub fn login(self) -> Url {
        self.base_url().add_hash_path_part(LOGIN_PART)
    }

    pub fn register(self) -> Url {
        self.base_url().add_hash_path_part(REGISTER_PART)
    }
}
