use serde::{Deserialize, Serialize};
use yewdux::prelude::{Persistent, PersistentStore};
use yewdux_functional::{use_store, StoreRef};

use crate::api::{ApiError, Task};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum AppState {
    Guest,
    GuestErr(ApiError),
    RequestedGuest(Task),
    RequestedGuestErr(Task, ApiError),
}

impl Default for AppState {
    fn default() -> Self {
        Self::Guest
    }
}

impl Default for &AppState {
    fn default() -> Self {
        &AppState::Guest
    }
}

impl Persistent for AppState {}

pub fn app_state_store() -> StoreRef<PersistentStore<AppState>> {
    use_store::<PersistentStore<AppState>>()
}
