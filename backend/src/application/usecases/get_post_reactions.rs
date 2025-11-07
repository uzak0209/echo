use crate::{
    application::error::AppError,
    domain::{entities::ReactionType, repositories::ReactionRepository},
};
use std::sync::Arc;
use uuid::Uuid;

pub struct GetPostReactionsUseCase {
    reaction_repository: Arc<dyn ReactionRepository>,
}

#[derive(Debug, Clone)]
pub struct ReactionCount {
    pub reaction_type: ReactionType,
    pub count: i64,
}

impl GetPostReactionsUseCase {
    pub fn new(reaction_repository: Arc<dyn ReactionRepository>) -> Self {
        Self {
            reaction_repository,
        }
    }

    pub async fn execute(&self, post_id: Uuid) -> Result<Vec<ReactionCount>, AppError> {
        let counts = self.reaction_repository.get_reaction_counts(post_id).await?;

        Ok(counts
            .into_iter()
            .map(|(reaction_type, count)| ReactionCount {
                reaction_type,
                count,
            })
            .collect())
    }
}
