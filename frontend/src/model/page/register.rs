use seed::prelude::{web_sys::HtmlInputElement, *};

pub struct RegisterPageData {
    pub user_name_ref: ElRef<HtmlInputElement>,
    pub password_ref: ElRef<HtmlInputElement>,
    pub invite_code_ref: ElRef<HtmlInputElement>,
    pub error: Option<String>,
}

impl RegisterPageData {
    pub fn new() -> Self {
        RegisterPageData {
            user_name_ref: ElRef::new(),
            password_ref: ElRef::new(),
            invite_code_ref: ElRef::new(),
            error: None,
        }
    }
}
