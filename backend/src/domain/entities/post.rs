use crate::domain::value_objects::{DisplayCount, PostContent};
use chrono::{DateTime, Utc};
use uuid::Uuid;

/// Post domain entity
#[derive(Debug, Clone)]
pub struct Post {
    pub id: Uuid,
    pub user_id: Uuid,
    pub content: PostContent,
    pub image_url: Option<String>,
    pub display_count: DisplayCount,
    pub created_at: DateTime<Utc>,
}

impl Post {
    pub fn new(user_id: Uuid, content: PostContent, image_url: Option<String>) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id,
            content,
            image_url,
            display_count: DisplayCount::new(),
            created_at: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

    #[fixture]
    fn sample_post_content() -> PostContent {
        PostContent::new("Test post content".to_string()).unwrap()
    }

    #[rstest]
    fn test_new_post(sample_post_content: PostContent) {
        let post = Post::new(uuid::Uuid::new_v4(), sample_post_content.clone(), None);

        assert_eq!(post.content.value(), "Test post content");
        assert_eq!(post.image_url, None);
        assert_eq!(post.display_count.value(), 0);
    }

    #[rstest]
    fn test_new_post_with_image(sample_post_content: PostContent) {
        let post = Post::new(
            uuid::Uuid::new_v4(),
            sample_post_content,
            Some("https://example.com/image.jpg".to_string()),
        );

        assert_eq!(
            post.image_url,
            Some("https://example.com/image.jpg".to_string())
        );
    }
}
