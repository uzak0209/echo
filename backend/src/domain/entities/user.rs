use chrono::{DateTime, Utc};

/// User domain entity
#[derive(Debug, Clone)]
pub struct User {
    pub id: UserId,
    pub display_name: DisplayName,
    pub avatar_url: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UserId(pub i32);

#[derive(Debug, Clone)]
pub struct DisplayName(String);

impl DisplayName {
    pub fn new(name: String) -> Self {
        Self(name)
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

impl User {
    pub fn new(id: UserId, display_name: DisplayName, avatar_url: String) -> Self {
        Self {
            id,
            display_name,
            avatar_url,
            created_at: Utc::now(),
        }
    }
}
