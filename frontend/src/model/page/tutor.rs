use core::matches;

use seed::prelude::*;

use super::{login::LoginPageData, register::RegisterPageData, Page};

pub enum TutorPage {
    Index { error: Option<String> },
    NotFound,
}

impl From<Url> for TutorPage {
    fn from(mut url: Url) -> Self {
        match url.remaining_path_parts().as_slice() {
            [] => TutorPage::Index { error: None },
            _ => TutorPage::NotFound,
        }
    }
}

impl Page for TutorPage {
    fn set_error_message(&mut self, message: String) {
        if let TutorPage::Index { error } = self {
            *error = Some(message)
        }
    }

    fn error_message(&self) -> Option<&String> {
        if let TutorPage::Index { error } = self {
            error.as_ref()
        } else {
            None
        }
    }

    fn is_not_found(&self) -> bool {
        matches!(self, TutorPage::NotFound)
    }

    fn login_data(&self) -> Option<&LoginPageData> {
        None
    }

    fn register_data(&self) -> Option<&RegisterPageData> {
        None
    }
}
