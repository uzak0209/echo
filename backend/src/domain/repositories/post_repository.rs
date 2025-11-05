use async_trait::async_trait;
use crate::domain::entities::{Post, PostId};

#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn find_by_id(&self, id: PostId) -> Result<Option<Post>, Box<dyn std::error::Error + Send + Sync>>;
    async fn find_all(&self) -> Result<Vec<Post>, Box<dyn std::error::Error + Send + Sync>>;
    async fn find_available(&self, limit: usize) -> Result<Vec<Post>, Box<dyn std::error::Error + Send + Sync>>;
    async fn save(&self, post: &Post) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
    async fn delete(&self, id: PostId) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}
