use std::collections::{HashMap, HashSet};

use seed::prelude::*;
use serde::{Deserialize, Serialize};

use crate::api::{admin::Invite, task::Task, user::ApiUser};

use super::page::{
    admin::AdminPage,
    guest::GuestPage,
    requested_guest::{RequestedGuestIndexData, RequestedGuestPage},
    tutor::TutorPage,
    Page,
};

pub enum User {
    Guest(GuestData),
    RequestedGuest(RequestedGuestData),
    Admin(AdminData),
    Tutor(TutorData),
}

pub struct GuestData(pub GuestPage);
pub struct RequestedGuestData {
    pub task: Task,
    pub page: RequestedGuestPage,
}

pub struct AdminData {
    pub token: String,
    pub invites: HashSet<Invite>,
    pub users: HashSet<ApiUser>,
    pub page: AdminPage,
}

pub struct TutorData {
    pub token: String,
    pub tasks: HashMap<String, Task>,
    pub page: TutorPage,
}

impl User {
    pub fn init(url: Url) -> Self {
        let saved_user = SavedUser::init();

        match saved_user {
            SavedUser::Guest => User::Guest(GuestData(url.into())),
            SavedUser::RequestedGuest(task) => User::RequestedGuest(RequestedGuestData {
                task,
                page: url.into(),
            }),
            SavedUser::Admin(token, invites, users) => User::Admin(AdminData {
                token,
                invites: invites.into_iter().collect(),
                users: users.into_iter().collect(),
                page: url.into(),
            }),
            SavedUser::Tutor(token, tasks) => User::Tutor(TutorData {
                token,
                tasks,
                page: url.into(),
            }),
        }
    }

    pub fn save(&self) {
        SavedUser::save(self)
    }

    pub fn change_page(&mut self, url: Url) {
        match self {
            User::Guest(data) => data.0 = url.into(),
            User::RequestedGuest(data) => data.page = url.into(),
            User::Admin(data) => data.page = url.into(),
            User::Tutor(data) => data.page = url.into(),
        }
    }

    pub fn page(&self) -> &dyn Page {
        match self {
            User::Guest(data) => &data.0,
            User::RequestedGuest(data) => &data.page,
            User::Admin(data) => &data.page,
            User::Tutor(data) => &data.page,
        }
    }

    pub fn page_mut(&mut self) -> &mut dyn Page {
        match self {
            User::Guest(data) => &mut data.0,
            User::RequestedGuest(data) => &mut data.page,
            User::Admin(data) => &mut data.page,
            User::Tutor(data) => &mut data.page,
        }
    }
    pub fn get_token(&self) -> Option<&String> {
        match self {
            User::Tutor(data) => Some(&data.token),
            User::Admin(data) => Some(&data.token),
            _ => None,
        }
    }

    pub fn as_guest<F: FnOnce(&mut GuestData)>(&mut self, func: F) {
        if let User::Guest(data) = self {
            func(data)
        }
    }

    pub fn as_admin<F: FnOnce(&mut AdminData)>(&mut self, func: F) {
        if let User::Admin(data) = self {
            func(data)
        }
    }

    pub fn as_requested_guest<F: FnOnce(&mut RequestedGuestData)>(&mut self, func: F) {
        if let User::RequestedGuest(data) = self {
            func(data)
        }
    }
}

impl RequestedGuestData {
    pub fn start_editing(&mut self) {
        if let RequestedGuestPage::Index(data) = &mut self.page {
            *data = RequestedGuestIndexData::Editing {
                title_ref: ElRef::new(),
                description_ref: ElRef::new(),
                error: None,
            }
        }
    }

    pub fn cancel_editing(&mut self) {
        if let RequestedGuestPage::Index(data) = &mut self.page {
            *data = RequestedGuestIndexData::Viewing { error: None }
        }
    }

    pub fn update_task(&mut self, new_task: Task) {
        self.task = new_task;

        if let RequestedGuestPage::Index(data) = &mut self.page {
            *data = RequestedGuestIndexData::Viewing { error: None }
        }
    }
}

impl AdminData {
    pub fn add_invite(&mut self, invite: Invite) {
        self.invites.insert(invite);
    }

    pub fn remove_invite(&mut self, invite: &Invite) {
        self.invites.remove(invite);
    }

    pub fn remove_user(&mut self, user: &ApiUser) {
        self.users.remove(user);
    }
}

// This enum is used to store the relevant data
// of the user in the local storage
// It is used to reconstruct the user
#[derive(Serialize, Deserialize)]
enum SavedUser {
    Guest,
    RequestedGuest(Task),
    Admin(String, Vec<Invite>, Vec<ApiUser>),
    Tutor(String, HashMap<String, Task>),
}

impl SavedUser {
    fn init() -> Self {
        LocalStorage::get(Self::storage_key()).unwrap_or(SavedUser::Guest)
    }

    fn save(user: &User) {
        let saved_user = match user {
            User::Guest(_) => SavedUser::Guest,
            User::RequestedGuest(data) => SavedUser::RequestedGuest(data.task.clone()),
            User::Admin(data) => SavedUser::Admin(
                data.token.clone(),
                data.invites.clone().into_iter().collect(),
                data.users.clone().into_iter().collect(),
            ),
            User::Tutor(data) => SavedUser::Tutor(data.token.clone(), data.tasks.clone()),
        };

        LocalStorage::insert(Self::storage_key(), &saved_user).expect("Can't save User")
    }

    #[inline]
    const fn storage_key() -> &'static str {
        "USER"
    }
}
