use seed::prelude::{
    web_sys::{HtmlInputElement, HtmlTextAreaElement},
    *,
};

use super::{login::LoginPageData, register::RegisterPageData, Page, LOGIN_PART, REGISTER_PART};

pub enum GuestPage {
    Index(GuestIndexData),
    Login(LoginPageData),
    Register(RegisterPageData),
    NotFound,
}

impl From<Url> for GuestPage {
    fn from(mut url: Url) -> Self {
        match url.remaining_path_parts().as_slice() {
            [] => GuestPage::Index(GuestIndexData::new()),
            [LOGIN_PART] => GuestPage::Login(LoginPageData::new()),
            [REGISTER_PART] => GuestPage::Register(RegisterPageData::new()),
            _ => GuestPage::NotFound,
        }
    }
}

impl Page for GuestPage {
    fn set_error_message(&mut self, error: String) {
        match self {
            GuestPage::Index(data) => data.error = Some(error),
            GuestPage::Login(data) => data.error = Some(error),
            GuestPage::NotFound => {}
            GuestPage::Register(data) => data.error = Some(error),
        }
    }

    fn error_message(&self) -> Option<&String> {
        match self {
            GuestPage::Index(data) => data.error.as_ref(),
            GuestPage::Login(data) => data.error.as_ref(),
            GuestPage::NotFound => None,
            GuestPage::Register(data) => data.error.as_ref(),
        }
    }

    fn is_not_found(&self) -> bool {
        matches!(self, GuestPage::NotFound)
    }

    fn login_data(&self) -> Option<&LoginPageData> {
        if let GuestPage::Login(data) = self {
            Some(data)
        } else {
            None
        }
    }

    fn register_data(&self) -> Option<&RegisterPageData> {
        if let GuestPage::Register(data) = self {
            Some(data)
        } else {
            None
        }
    }
}

pub struct GuestIndexData {
    pub title_ref: ElRef<HtmlInputElement>,
    pub description_ref: ElRef<HtmlTextAreaElement>,
    pub error: Option<String>,
}

impl GuestIndexData {
    pub fn new() -> Self {
        GuestIndexData {
            title_ref: ElRef::new(),
            description_ref: ElRef::new(),
            error: None,
        }
    }
}
