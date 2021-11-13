use serde::{Deserialize, Serialize};
use yewdux::prelude::{Persistent, PersistentStore};
use yewdux_functional::{use_store, StoreRef};

use crate::api::{tasks::Task, ApiError};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum AppState {
    Guest(Option<ApiError>),
    RequestedGuest(Task, Option<ApiError>),
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

pub fn app_state_store() -> StoreRef<PersistentStore<AppState>> {
    use_store::<PersistentStore<AppState>>()
}
