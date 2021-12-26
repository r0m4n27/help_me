use core::matches;

use seed::prelude::*;

use super::Page;

pub enum AdminPage {
    Index,
    NotFound,
}

impl From<Url> for AdminPage {
    fn from(mut url: Url) -> Self {
        match url.remaining_path_parts().as_slice() {
            [] => AdminPage::Index,
            _ => AdminPage::NotFound,
        }
    }
}

impl Page for AdminPage {
    fn set_error_message(&mut self, _: String) {}

    fn error_message(&self) -> Option<&String> {
        None
    }

    fn is_not_found(&self) -> bool {
        matches!(self, AdminPage::NotFound)
    }

    fn login_data(&self) -> Option<&super::login::LoginPageData> {
        None
    }

    fn register_data(&self) -> Option<&super::register::RegisterPageData> {
        None
    }
}
