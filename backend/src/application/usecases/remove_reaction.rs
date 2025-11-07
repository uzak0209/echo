use crate::{
    application::error::AppError,
    domain::{entities::ReactionType, repositories::ReactionRepository},
};
use std::sync::Arc;
use uuid::Uuid;

pub struct RemoveReactionUseCase {
    reaction_repository: Arc<dyn ReactionRepository>,
}

impl RemoveReactionUseCase {
    pub fn new(reaction_repository: Arc<dyn ReactionRepository>) -> Self {
        Self {
            reaction_repository,
        }
    }

    pub async fn execute(
        &self,
        post_id: Uuid,
        user_id: Uuid,
        reaction_type: ReactionType,
    ) -> Result<bool, AppError> {
        self.reaction_repository
            .remove_reaction(post_id, user_id, reaction_type)
            .await?;

        Ok(true)
    }
}
