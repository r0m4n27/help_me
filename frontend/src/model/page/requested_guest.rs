use seed::prelude::{
    web_sys::{HtmlInputElement, HtmlTextAreaElement},
    *,
};

use super::{login::LoginPageData, register::RegisterPageData, Page, LOGIN_PART, REGISTER_PART};

pub enum RequestedGuestPage {
    Index(RequestedGuestIndexData),
    Login(LoginPageData),
    Register(RegisterPageData),
    NotFound,
}

impl From<Url> for RequestedGuestPage {
    fn from(mut url: Url) -> Self {
        match url.remaining_path_parts().as_slice() {
            [] => RequestedGuestPage::Index(RequestedGuestIndexData::Viewing { error: None }),
            [LOGIN_PART] => RequestedGuestPage::Login(LoginPageData::new()),
            [REGISTER_PART] => RequestedGuestPage::Register(RegisterPageData::new()),
            _ => RequestedGuestPage::NotFound,
        }
    }
}

impl Page for RequestedGuestPage {
    fn set_error_message(&mut self, error: String) {
        match self {
            RequestedGuestPage::Index(RequestedGuestIndexData::Editing { error: err, .. }) => {
                *err = Some(error)
            }
            RequestedGuestPage::Index(RequestedGuestIndexData::Viewing { error: err, .. }) => {
                *err = Some(error)
            }
            RequestedGuestPage::Login(data) => data.error = Some(error),
            RequestedGuestPage::NotFound => {}
            RequestedGuestPage::Register(data) => data.error = Some(error),
        }
    }

    fn error_message(&self) -> Option<&String> {
        match self {
            RequestedGuestPage::Index(RequestedGuestIndexData::Editing { error, .. }) => {
                error.as_ref()
            }
            RequestedGuestPage::Index(RequestedGuestIndexData::Viewing { error, .. }) => {
                error.as_ref()
            }
            RequestedGuestPage::Login(data) => data.error.as_ref(),
            RequestedGuestPage::NotFound => None,
            RequestedGuestPage::Register(data) => data.error.as_ref(),
        }
    }

    fn is_not_found(&self) -> bool {
        matches!(self, RequestedGuestPage::NotFound)
    }

    fn login_data(&self) -> Option<&LoginPageData> {
        if let RequestedGuestPage::Login(data) = self {
            Some(data)
        } else {
            None
        }
    }

    fn register_data(&self) -> Option<&RegisterPageData> {
        if let RequestedGuestPage::Register(data) = self {
            Some(data)
        } else {
            None
        }
    }
}

pub enum RequestedGuestIndexData {
    Viewing {
        error: Option<String>,
    },
    Editing {
        title_ref: ElRef<HtmlInputElement>,
        description_ref: ElRef<HtmlTextAreaElement>,
        error: Option<String>,
    },
}
