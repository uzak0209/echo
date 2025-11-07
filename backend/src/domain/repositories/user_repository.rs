use crate::domain::{entities::User, error::DomainError};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError>;
    async fn find_by_email(&self, email: &str) -> Result<Option<User>, DomainError>;
    async fn create_user(
        &self,
        display_name: String,
        avatar_url: Option<String>,
    ) -> Result<User, DomainError>;
    async fn create_user_with_credentials(
        &self,
        display_name: String,
        avatar_url: Option<String>,
        email: String,
        password_hash: String,
    ) -> Result<User, DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
}
