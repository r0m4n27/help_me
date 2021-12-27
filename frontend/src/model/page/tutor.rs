use core::matches;
use std::collections::HashMap;

use seed::prelude::*;

use crate::api::task::Task;

use super::{
    login::LoginPageData, register::RegisterPageData, settings::SettingsPageData, Page,
    SETTINGS_PART, TASK_PART,
};

pub enum TutorPage {
    Index {
        error: Option<String>,
    },
    Settings(SettingsPageData),
    Task {
        task_id: String,
        error: Option<String>,
    },
    NotFound,
}

impl TutorPage {
    pub fn new(mut url: Url, tasks: &HashMap<String, Task>) -> Self {
        match url.remaining_path_parts().as_slice() {
            [] => TutorPage::Index { error: None },
            [SETTINGS_PART] => TutorPage::Settings(SettingsPageData::new()),
            [TASK_PART, task_id] => {
                let task_id = task_id.to_string();

                if tasks.contains_key(&task_id) {
                    TutorPage::Task {
                        error: None,
                        task_id,
                    }
                } else {
                    TutorPage::NotFound
                }
            }
            _ => TutorPage::NotFound,
        }
    }
}

impl Page for TutorPage {
    fn set_error_message(&mut self, message: String) {
        match self {
            TutorPage::Index { error } => *error = Some(message),
            TutorPage::Task { error, .. } => *error = Some(message),
            TutorPage::NotFound => {}
            TutorPage::Settings(data) => data.error = Some(message),
        }
    }

    fn error_message(&self) -> Option<&String> {
        match self {
            TutorPage::Index { error } => error.as_ref(),
            TutorPage::Task { error, .. } => error.as_ref(),
            TutorPage::NotFound => None,
            TutorPage::Settings(data) => data.error.as_ref(),
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

    fn settings_data(&self) -> Option<&SettingsPageData> {
        if let TutorPage::Settings(data) = self {
            Some(data)
        } else {
            None
        }
    }
}
