use seed::prelude::{
    web_sys::{HtmlInputElement, HtmlTextAreaElement},
    *,
};

use super::{login::LoginPageData, Page, LOGIN_PART};

pub enum RequestedGuestPage {
    Index(RequestedGuestIndexData),
    Login(LoginPageData),
    NotFound,
}

impl From<Url> for RequestedGuestPage {
    fn from(mut url: Url) -> Self {
        match url.remaining_path_parts().as_slice() {
            [] => RequestedGuestPage::Index(RequestedGuestIndexData::Viewing { error: None }),
            [LOGIN_PART] => RequestedGuestPage::Login(LoginPageData::new()),
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
        }
    }

    fn is_not_found(&self) -> bool {
        matches!(self, RequestedGuestPage::NotFound)
    }

    fn login_data(&self) -> Option<&LoginPageData> {
        match self {
            RequestedGuestPage::Index(_) => None,
            RequestedGuestPage::Login(data) => Some(data),
            RequestedGuestPage::NotFound => None,
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
