use serde::{Deserialize, Serialize};
use yewdux::prelude::{Persistent, PersistentStore};
use yewdux_functional::{use_store, StoreRef};

use crate::api::Task;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum AppState {
    Guest,
    RequestedGuest(Task),
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
