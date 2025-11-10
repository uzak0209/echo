use crate::domain::{entities::User, error::DomainError};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: Uuid) -> Result<Option<User>, DomainError>;
    async fn find_by_username(&self, username: &str) -> Result<Option<User>, DomainError>;
    async fn create_user(
        &self,
        display_name: String,
        avatar_url: Option<String>,
    ) -> Result<User, DomainError>;
    async fn create_user_with_credentials(
        &self,
        display_name: String,
        avatar_url: Option<String>,
        password_hash: String,
    ) -> Result<User, DomainError>;
    async fn update_refresh_token(
        &self,
        user_id: Uuid,
        refresh_token: Option<String>,
    ) -> Result<(), DomainError>;
    async fn delete(&self, id: Uuid) -> Result<(), DomainError>;
}
