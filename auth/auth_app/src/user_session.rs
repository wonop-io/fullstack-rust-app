use auth_api::User;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserSession {
    pub user: Option<User>,
    pub login_time: DateTime<Utc>,
}

impl UserSession {
    pub fn new(user: Option<User>) -> Self {
        Self {
            user,
            login_time: Utc::now(),
        }
    }
}
