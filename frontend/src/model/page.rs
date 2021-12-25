use seed::prelude::{
    web_sys::{HtmlInputElement, HtmlTextAreaElement},
    *,
};

use crate::api::task::Task;

const LOGIN_PART: &str = "login";
// const REGISTER_PART: &str = "register";
// const TASK_PART: &str = "task";

pub struct Urls {
    pub base_url: Url,
}

impl Urls {
    pub fn new(base_url: Url) -> Self {
        Urls { base_url }
    }

    pub fn index(&self) -> Url {
        self.base_url.clone()
    }

    pub fn login(&self) -> Url {
        self.index().add_path_part(LOGIN_PART)
    }

    pub fn goto_index(&self) {
        self.base_url.go_and_replace()
    }
}

pub trait Page {
    fn set_error_message(&mut self, error: String);
    fn error_message(&self) -> Option<&String>;
    fn not_found(&self) -> bool;
}

pub enum GuestPages {
    Index {
        title_ref: ElRef<HtmlInputElement>,
        description_ref: ElRef<HtmlTextAreaElement>,
        error: Option<String>,
    },
    NotFound,
}

impl GuestPages {
    pub fn new(mut url: Url) -> Self {
        match url.remaining_path_parts().as_slice() {
            [] => GuestPages::Index {
                title_ref: ElRef::new(),
                description_ref: ElRef::new(),
                error: None,
            },
            _ => GuestPages::NotFound,
        }
    }
}

impl Page for GuestPages {
    fn set_error_message(&mut self, error: String) {
        match self {
            GuestPages::Index { error: err, .. } => *err = Some(error),
            GuestPages::NotFound => {}
        }
    }

    fn error_message(&self) -> Option<&String> {
        match self {
            GuestPages::Index { error, .. } => error.as_ref(),
            GuestPages::NotFound => None,
        }
    }

    fn not_found(&self) -> bool {
        matches!(self, GuestPages::NotFound)
    }
}

pub enum RequestedGuestPages {
    Index {
        page_data: RequestedGuestIndexData,
        error: Option<String>,
    },
    NotFound,
}

impl RequestedGuestPages {
    pub fn new(mut url: Url, task: &Task) -> Self {
        match url.remaining_path_parts().as_slice() {
            [] => RequestedGuestPages::Index {
                page_data: RequestedGuestIndexData::Viewing {
                    title: task.title.clone(),
                    description: task.body.clone(),
                },
                error: None,
            },
            _ => RequestedGuestPages::NotFound,
        }
    }
}

impl Page for RequestedGuestPages {
    fn set_error_message(&mut self, error: String) {
        match self {
            RequestedGuestPages::Index { error: err, .. } => *err = Some(error),
            RequestedGuestPages::NotFound => {}
        }
    }

    fn error_message(&self) -> Option<&String> {
        match self {
            RequestedGuestPages::Index { error, .. } => error.as_ref(),
            RequestedGuestPages::NotFound => None,
        }
    }

    fn not_found(&self) -> bool {
        matches!(self, RequestedGuestPages::NotFound)
    }
}

pub enum RequestedGuestIndexData {
    Viewing {
        title: String,
        description: String,
    },
    Editing {
        title_ref: ElRef<HtmlInputElement>,
        description_ref: ElRef<HtmlTextAreaElement>,
    },
}
