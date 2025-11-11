use crate::domain::{entities::{Post, User}, error::DomainError};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait PostRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<Post>, DomainError>;
    async fn find_available_with_users(
        &self,
        limit: usize,
        exclude_user_id: Option<Uuid>,
    ) -> Result<Vec<(Post, User)>, DomainError>;
    async fn create(&self, post: &Post) -> Result<Post, DomainError>;
    async fn increment_display_count(&self, id: Uuid) -> Result<Post, DomainError>;
}
