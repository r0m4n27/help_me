use core::matches;

use seed::prelude::*;

use super::{
    login::LoginPageData, register::RegisterPageData, settings::SettingsPageData, Page,
    SETTINGS_PART,
};

pub enum AdminPage {
    Index { error: Option<String> },
    Settings(SettingsPageData),
    NotFound,
}

impl From<Url> for AdminPage {
    fn from(mut url: Url) -> Self {
        match url.remaining_path_parts().as_slice() {
            [] => AdminPage::Index { error: None },
            [SETTINGS_PART] => AdminPage::Settings(SettingsPageData::new()),
            _ => AdminPage::NotFound,
        }
    }
}

impl Page for AdminPage {
    fn set_error_message(&mut self, message: String) {
        match self {
            AdminPage::Index { error } => *error = Some(message),
            AdminPage::Settings(data) => data.error = Some(message),
            AdminPage::NotFound => {}
        }
    }

    fn error_message(&self) -> Option<&String> {
        match self {
            AdminPage::Index { error } => error.as_ref(),
            AdminPage::Settings(data) => data.error.as_ref(),
            AdminPage::NotFound => None,
        }
    }

    fn is_not_found(&self) -> bool {
        matches!(self, AdminPage::NotFound)
    }

    fn login_data(&self) -> Option<&LoginPageData> {
        None
    }

    fn register_data(&self) -> Option<&RegisterPageData> {
        None
    }

    fn settings_data(&self) -> Option<&SettingsPageData> {
        if let AdminPage::Settings(data) = self {
            Some(data)
        } else {
            None
        }
    }
}
