use crate::domain::{entities::{Reaction, ReactionType}, error::DomainError};
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait ReactionRepository: Send + Sync {
    /// Add or update a reaction to a post
    async fn add_reaction(
        &self,
        post_id: Uuid,
        user_id: Uuid,
        reaction_type: ReactionType,
    ) -> Result<Reaction, DomainError>;

    /// Remove a reaction from a post
    async fn remove_reaction(
        &self,
        post_id: Uuid,
        user_id: Uuid,
    ) -> Result<(), DomainError>;

    /// Get the latest reaction across all posts by a specific user (for displaying on user's avatar)
    async fn get_latest_reaction_for_user(&self, user_id: Uuid) -> Result<Option<Reaction>, DomainError>;
}
