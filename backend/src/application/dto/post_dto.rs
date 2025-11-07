use uuid::Uuid;

use crate::domain::entities::Post;

/// Data Transfer Object for Post
#[derive(Debug, Clone)]
pub struct PostDto {
    pub id: Uuid,
    pub user_id: Uuid,
    pub content: String,
    pub image_url: Option<String>,
    pub author_name: String,
    pub author_avatar: String,
}

impl PostDto {
    pub fn new(
        post: Post,
        author_name: String,
        author_avatar: String,
    ) -> Self {
        Self {
            id: post.id,
            user_id: post.user_id,
            content: post.content.value().to_string(),
            image_url: post.image_url,
            author_name,
            author_avatar,
        }
    }
}

impl From<Post> for PostDto {
    fn from(post: Post) -> Self {
        Self {
            id: post.id,
            user_id: post.user_id,
            content: post.content.value().to_string(),
            image_url: post.image_url,
            author_name: String::new(), // Fallback
            author_avatar: String::new(), // Fallback
        }
    }
}
