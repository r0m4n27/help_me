use seed::prelude::{
    web_sys::{HtmlInputElement, HtmlTextAreaElement},
    *,
};

use super::{login::LoginPageData, Page, LOGIN_PART};

pub enum GuestPage {
    Index(GuestIndexData),
    Login(LoginPageData),
    NotFound,
}

impl From<Url> for GuestPage {
    fn from(mut url: Url) -> Self {
        match url.remaining_path_parts().as_slice() {
            [] => GuestPage::Index(GuestIndexData::new()),
            [LOGIN_PART] => GuestPage::Login(LoginPageData::new()),
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
        }
    }

    fn error_message(&self) -> Option<&String> {
        match self {
            GuestPage::Index(data) => data.error.as_ref(),
            GuestPage::Login(data) => data.error.as_ref(),
            GuestPage::NotFound => None,
        }
    }

    fn is_not_found(&self) -> bool {
        matches!(self, GuestPage::NotFound)
    }

    fn login_data(&self) -> Option<&LoginPageData> {
        match self {
            GuestPage::Index(_) => None,
            GuestPage::Login(data) => Some(data),
            GuestPage::NotFound => None,
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
