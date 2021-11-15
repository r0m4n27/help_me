use serde::{Deserialize, Serialize};
use yewdux::prelude::{Persistent, PersistentStore};

use crate::api::{tasks::Task, ApiError};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum AppState {
    Guest(Option<ApiError>),
    RequestedGuest(Task, Option<ApiError>),
    Tutor(String),
    Admin(String),
}

impl Default for AppState {
    fn default() -> Self {
        Self::Guest(None)
    }
}

impl Default for &AppState {
    fn default() -> Self {
        &AppState::Guest(None)
    }
}

impl Persistent for AppState {}

pub type AppStateStore = PersistentStore<AppState>;
