use chrono::{DateTime, Utc};
use crate::domain::value_objects::DisplayName;

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
        let user = User::new(
            UserId(1),
            sample_display_name.clone(),
            sample_avatar_url.clone(),
        );

        assert_eq!(user.id, UserId(1));
        assert_eq!(user.display_name.value(), "TestUser");
        assert_eq!(user.avatar_url, sample_avatar_url);
    }

    #[rstest]
    #[case(UserId(1), "User1", "https://example.com/1.jpg")]
    #[case(UserId(42), "User42", "https://example.com/42.jpg")]
    #[case(UserId(999), "Anonymous", "https://example.com/default.jpg")]
    fn test_new_user_with_various_data(
        #[case] id: UserId,
        #[case] name: &str,
        #[case] avatar: &str,
    ) {
        let user = User::new(
            id,
            DisplayName::new(name.to_string()),
            avatar.to_string(),
        );

        assert_eq!(user.id, id);
        assert_eq!(user.display_name.value(), name);
        assert_eq!(user.avatar_url, avatar);
    }

    #[rstest]
    #[case(UserId(1))]
    #[case(UserId(42))]
    #[case(UserId(999))]
    fn test_user_id_equality(#[case] id: UserId) {
        let id2 = id;
        assert_eq!(id, id2);
    }
}
