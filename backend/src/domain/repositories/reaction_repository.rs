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
        reaction_type: ReactionType,
    ) -> Result<(), DomainError>;

    /// Get all reactions for a specific post
    async fn find_by_post_id(&self, post_id: Uuid) -> Result<Vec<Reaction>, DomainError>;

    /// Get reaction counts by type for a post
    async fn get_reaction_counts(&self, post_id: Uuid) -> Result<Vec<(ReactionType, i64)>, DomainError>;

    /// Get the latest reaction for each post by the author (for displaying on author's avatar)
    async fn get_latest_reaction_for_post(&self, post_id: Uuid) -> Result<Option<Reaction>, DomainError>;

    /// Get the latest reaction across all posts by a specific user (for displaying on user's avatar)
    async fn get_latest_reaction_for_user(&self, user_id: Uuid) -> Result<Option<Reaction>, DomainError>;
}
