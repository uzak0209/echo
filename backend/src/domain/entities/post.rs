use chrono::{DateTime, Utc};

/// Post domain entity
#[derive(Debug, Clone)]
pub struct Post {
    pub id: PostId,
    pub user_id: i32,
    pub content: PostContent,
    pub image_url: Option<String>,
    pub display_count: DisplayCount,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PostId(pub i32);

#[derive(Debug, Clone)]
pub struct PostContent(String);

impl PostContent {
    pub fn new(content: String) -> Result<Self, String> {
        if content.is_empty() {
            return Err("Content cannot be empty".to_string());
        }
        if content.len() > 1000 {
            return Err("Content too long (max 1000 characters)".to_string());
        }
        Ok(Self(content))
    }

    pub fn value(&self) -> &str {
        &self.0
    }
}

#[derive(Debug, Clone, Copy)]
pub struct DisplayCount(i32);

impl DisplayCount {
    pub fn new() -> Self {
        Self(0)
    }

    pub fn from_value(value: i32) -> Self {
        Self(value)
    }

    pub fn increment(&mut self) {
        self.0 += 1;
    }

    pub fn value(&self) -> i32 {
        self.0
    }

    pub fn is_expired(&self) -> bool {
        self.0 >= 10
    }
}

impl Default for DisplayCount {
    fn default() -> Self {
        Self::new()
    }
}

impl Post {
    pub fn new(
        id: PostId,
        user_id: i32,
        content: PostContent,
        image_url: Option<String>,
    ) -> Self {
        Self {
            id,
            user_id,
            content,
            image_url,
            display_count: DisplayCount::new(),
            created_at: Utc::now(),
        }
    }

    pub fn increment_display(&mut self) {
        self.display_count.increment();
    }

    pub fn is_expired(&self) -> bool {
        self.display_count.is_expired()
    }
}
