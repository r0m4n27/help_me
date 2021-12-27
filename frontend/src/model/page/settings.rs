use seed::prelude::{web_sys::HtmlInputElement, *};

pub struct SettingsPageData {
    pub user_name_ref: ElRef<HtmlInputElement>,
    pub user_name_again_ref: ElRef<HtmlInputElement>,

    pub password_ref: ElRef<HtmlInputElement>,
    pub password_again_ref: ElRef<HtmlInputElement>,

    pub error: Option<String>,
}

impl SettingsPageData {
    pub fn new() -> Self {
        SettingsPageData {
            user_name_ref: ElRef::new(),
            password_ref: ElRef::new(),
            error: None,
            user_name_again_ref: ElRef::new(),
            password_again_ref: ElRef::new(),
        }
    }
}
