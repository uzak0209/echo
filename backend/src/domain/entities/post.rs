use chrono::{DateTime, Utc};
use crate::domain::value_objects::{PostContent, DisplayCount};

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
        let post = Post::new(
            PostId(1),
            123,
            sample_post_content.clone(),
            None,
        );

        assert_eq!(post.id, PostId(1));
        assert_eq!(post.user_id, 123);
        assert_eq!(post.content.value(), "Test post content");
        assert_eq!(post.image_url, None);
        assert_eq!(post.display_count.value(), 0);
        assert!(!post.is_expired());
    }

    #[rstest]
    fn test_new_post_with_image(sample_post_content: PostContent) {
        let post = Post::new(
            PostId(1),
            123,
            sample_post_content,
            Some("https://example.com/image.jpg".to_string()),
        );

        assert_eq!(post.image_url, Some("https://example.com/image.jpg".to_string()));
    }

    #[rstest]
    fn test_increment_display(sample_post_content: PostContent) {
        let mut post = Post::new(PostId(1), 123, sample_post_content, None);

        assert_eq!(post.display_count.value(), 0);

        post.increment_display();
        assert_eq!(post.display_count.value(), 1);

        post.increment_display();
        assert_eq!(post.display_count.value(), 2);
    }

    #[rstest]
    fn test_is_expired_after_10_displays(sample_post_content: PostContent) {
        let mut post = Post::new(PostId(1), 123, sample_post_content, None);

        assert!(!post.is_expired());

        for _ in 0..9 {
            post.increment_display();
        }
        assert!(!post.is_expired());

        post.increment_display();
        assert!(post.is_expired());
    }

    #[rstest]
    #[case(PostId(1))]
    #[case(PostId(42))]
    #[case(PostId(999))]
    fn test_post_id_equality(#[case] id: PostId) {
        let id2 = id;
        assert_eq!(id, id2);
    }
}
