use async_trait::async_trait;
use crate::domain::entities::{User, UserId};

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: UserId) -> Result<Option<User>, Box<dyn std::error::Error + Send + Sync>>;
    async fn find_all(&self) -> Result<Vec<User>, Box<dyn std::error::Error + Send + Sync>>;
    async fn save(&self, user: &User) -> Result<UserId, Box<dyn std::error::Error + Send + Sync>>;
    async fn get_random(&self) -> Result<Option<User>, Box<dyn std::error::Error + Send + Sync>>;
}
