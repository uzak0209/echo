use async_trait::async_trait;
use crate::domain::{entities::{User, UserId}, error::DomainError};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: UserId) -> Result<Option<User>, DomainError>;
    async fn find_all(&self) -> Result<Vec<User>, DomainError>;
    async fn save(&self, user: &User) -> Result<UserId, DomainError>;
    async fn get_random(&self) -> Result<Option<User>, DomainError>;
}
