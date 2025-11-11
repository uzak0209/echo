use crate::domain::value_objects::DisplayName;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// User domain entity
#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub display_name: DisplayName,
    pub avatar_url: String,
    pub password_hash: Option<String>,
    pub refresh_token: Option<String>,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn new_with_credentials(
        display_name: DisplayName,
        avatar_url: String,
        password_hash: String,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            display_name,
            avatar_url,
            password_hash: Some(password_hash),
            refresh_token: None,
            created_at: Utc::now(),
        }
    }
}
