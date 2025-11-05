use crate::domain::value_objects::DisplayName;
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// User domain entity
#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub display_name: DisplayName,
    pub avatar_url: String,
    pub created_at: DateTime<Utc>,
}

impl User {
    pub fn new(display_name: DisplayName, avatar_url: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            display_name,
            avatar_url,
            created_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn sample_display_name() -> DisplayName {
        DisplayName::new("TestUser".to_string())
    }

    #[fixture]
    fn sample_avatar_url() -> String {
        "https://example.com/avatar.jpg".to_string()
    }

    #[rstest]
    fn test_new_user(sample_display_name: DisplayName, sample_avatar_url: String) {
        let user = User::new(sample_display_name.clone(), sample_avatar_url.clone());

        assert_eq!(user.display_name.value(), "TestUser");
        assert_eq!(user.avatar_url, sample_avatar_url);
    }

    #[rstest]
    #[case("User1", "https://example.com/1.jpg")]
    #[case("User42", "https://example.com/42.jpg")]
    #[case("Anonymous", "https://example.com/default.jpg")]
    fn test_new_user_with_various_data(#[case] name: &str, #[case] avatar: &str) {
        let user = User::new(DisplayName::new(name.to_string()), avatar.to_string());

        assert_eq!(user.display_name.value(), name);
        assert_eq!(user.avatar_url, avatar);
    }
}
