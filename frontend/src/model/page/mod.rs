use seed::prelude::*;

use self::{login::LoginPageData, register::RegisterPageData};

pub mod guest;
pub mod login;
pub mod register;
pub mod requested_guest;

const LOGIN_PART: &str = "login";
const REGISTER_PART: &str = "register";
// const TASK_PART: &str = "task";

pub struct Urls {
    pub base_url: Url,
}

impl Urls {
    pub fn new(base_url: Url) -> Self {
        Urls { base_url }
    }

    pub fn index(&self) -> Url {
        self.base_url.clone()
    }

    pub fn login(&self) -> Url {
        self.index().add_path_part(LOGIN_PART)
    }

    pub fn register(&self) -> Url {
        self.index().add_path_part(REGISTER_PART)
    }

    pub fn goto_index(&self) {
        self.base_url.go_and_replace()
    }
}

// Extensions from 'From<Url>' is not possible
// because otherwise it couldn't be used as a trait object
pub trait Page {
    fn set_error_message(&mut self, error: String);
    fn error_message(&self) -> Option<&String>;
    fn is_not_found(&self) -> bool;
    fn login_data(&self) -> Option<&LoginPageData>;
    fn register_data(&self) -> Option<&RegisterPageData>;
}
