// Boilerplate code by Wonop ApS.

use auth_api::User;
use serde::{Deserialize, Serialize};
use yewdux::prelude::*;

#[derive(Clone, PartialEq, Serialize, Deserialize, Default)]
pub enum AuthCheckState {
    #[default]
    Loading,
    Authenticated,
    Unauthenticated,
}

#[derive(Clone, PartialEq, Serialize, Deserialize, Default, Store)]
#[store(storage = "local")]
pub struct AuthStore {
    #[serde(skip)]
    pub check_state: AuthCheckState,
    pub email: Option<String>,
    pub user: Option<User>,
    pub error: Option<String>,
}

impl AuthStore {
    pub fn is_authenticated(&self) -> bool {
        self.user.is_some()
    }
    pub fn has_role(&self, role: &str) -> bool {
        self.user.as_ref().is_some_and(|user| user.role == role)
    }
}
