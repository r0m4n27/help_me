use std::{collections::HashMap, rc::Rc};

use serde::{Deserialize, Serialize};
use yewdux::prelude::{BasicStore, Persistent, PersistentStore, Store};
use yewdux_functional::StoreRef;

use crate::api::{admin::Invite, tasks::Task, user::User};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum AppState {
    Guest,
    RequestedGuest(Task),
    Tutor(String, HashMap<String, Task>),
    Admin(String, Vec<Invite>, Vec<User>),
}

impl Default for AppState {
    fn default() -> Self {
        Self::Guest
    }
}

impl Persistent for AppState {}

pub type AppStateStore = PersistentStore<AppState>;

pub struct StateWrapper<T>(Rc<T>);

impl<T> From<&Rc<T>> for StateWrapper<T> {
    fn from(data: &Rc<T>) -> Self {
        Self(data.clone())
    }
}

impl<T: Default> Default for StateWrapper<T> {
    fn default() -> Self {
        StateWrapper(Rc::new(Default::default()))
    }
}

pub trait GetState<T> {
    fn get_state(self) -> Rc<T>;
}

impl<T> GetState<T::Model> for StoreRef<T>
where
    T: Store,
    T::Model: Default,
{
    fn get_state(self) -> Rc<T::Model> {
        self.state().map(StateWrapper::from).unwrap_or_default().0
    }
}

#[derive(Clone, PartialEq)]
pub struct IndexErrorState(pub Option<String>);

impl Default for IndexErrorState {
    fn default() -> Self {
        Self(Default::default())
    }
}

pub type IndexErrorStateStore = BasicStore<IndexErrorState>;

#[derive(Clone, PartialEq)]
pub struct LoginErrorState(pub Option<String>);

impl Default for LoginErrorState {
    fn default() -> Self {
        Self(Default::default())
    }
}

pub type LoginErrorStateStore = BasicStore<LoginErrorState>;

#[derive(Clone, PartialEq)]
pub struct RegisterErrorState(pub Option<String>);

impl Default for RegisterErrorState {
    fn default() -> Self {
        Self(Default::default())
    }
}

pub type RegisterErrorStateStore = BasicStore<RegisterErrorState>;
