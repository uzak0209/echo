use uuid::Uuid;

use crate::domain::entities::Post;

/// Data Transfer Object for Post
#[derive(Debug, Clone)]
pub struct PostDto {
    pub id: Uuid,
    pub content: String,
    pub image_url: Option<String>,
}

impl From<Post> for PostDto {
    fn from(post: Post) -> Self {
        Self {
            id: post.id,
            content: post.content.value().to_string(),
            image_url: post.image_url,
        }
    }
}
