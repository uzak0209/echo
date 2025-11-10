use crate::{
    application::error::AppError,
    domain::repositories::ReactionRepository,
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
    ) -> Result<bool, AppError> {
        self.reaction_repository
            .remove_reaction(post_id, user_id)
            .await?;

        Ok(true)
    }
}
