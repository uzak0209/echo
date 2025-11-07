use crate::{
    application::error::AppError,
    domain::{entities::ReactionType, repositories::ReactionRepository},
};
use std::sync::Arc;
use uuid::Uuid;

pub struct GetUserLatestReactionUseCase {
    reaction_repository: Arc<dyn ReactionRepository>,
}

impl GetUserLatestReactionUseCase {
    pub fn new(reaction_repository: Arc<dyn ReactionRepository>) -> Self {
        Self {
            reaction_repository,
        }
    }

    /// Get the latest reaction type for a user (to display on their avatar)
    pub async fn execute(&self, user_id: Uuid) -> Result<Option<ReactionType>, AppError> {
        let reaction = self
            .reaction_repository
            .get_latest_reaction_for_user(user_id)
            .await?;

        Ok(reaction.map(|r| r.reaction_type))
    }
}
