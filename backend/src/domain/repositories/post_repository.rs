use async_trait::async_trait;
use crate::domain::{entities::{Post, PostId}, error::DomainError};

#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn find_by_id(&self, id: PostId) -> Result<Option<Post>, DomainError>;
    async fn find_all(&self) -> Result<Vec<Post>, DomainError>;
    async fn find_available(&self, limit: usize) -> Result<Vec<Post>, DomainError>;
    async fn save(&self, post: &Post) -> Result<(), DomainError>;
    async fn delete(&self, id: PostId) -> Result<(), DomainError>;
}
