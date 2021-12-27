use seed::prelude::{web_sys::HtmlInputElement, *};

pub struct LoginPageData {
    pub user_name_ref: ElRef<HtmlInputElement>,
    pub password_ref: ElRef<HtmlInputElement>,
    pub error: Option<String>,
}

impl LoginPageData {
    pub fn new() -> Self {
        LoginPageData {
            user_name_ref: ElRef::new(),
            password_ref: ElRef::new(),
            error: None,
        }
    }
}
