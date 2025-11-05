use crate::domain::{entities::User, error::DomainError};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError>;
    async fn save(&self, user: &User) -> Result<User, DomainError>;
    async fn create_user(
        &self,
        display_name: String,
        avatar_url: Option<String>,
    ) -> Result<User, DomainError>;
}
