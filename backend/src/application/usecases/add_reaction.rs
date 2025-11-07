use crate::{
    application::error::AppError,
    domain::{entities::ReactionType, repositories::{PostRepository, ReactionRepository}},
    infrastructure::sse::ReactionStreamManager,
};
use std::sync::Arc;
use uuid::Uuid;

pub struct AddReactionUseCase {
    reaction_repository: Arc<dyn ReactionRepository>,
    post_repository: Arc<dyn PostRepository>,
    stream_manager: Arc<ReactionStreamManager>,
}

impl AddReactionUseCase {
    pub fn new(
        reaction_repository: Arc<dyn ReactionRepository>,
        post_repository: Arc<dyn PostRepository>,
        stream_manager: Arc<ReactionStreamManager>,
    ) -> Self {
        Self {
            reaction_repository,
            post_repository,
            stream_manager,
        }
    }

    pub async fn execute(
        &self,
        post_id: Uuid,
        user_id: Uuid,
        reaction_type: ReactionType,
    ) -> Result<bool, AppError> {
        // Add reaction to database
        self.reaction_repository
            .add_reaction(post_id, user_id, reaction_type.clone())
            .await?;

        // Get post author to send SSE event
        if let Some(post) = self.post_repository.find_by_id(post_id).await? {
            // Broadcast to post author's SSE stream
            let _ = self
                .stream_manager
                .broadcast_reaction(post.user_id, post_id, user_id, reaction_type)
                .await;
        }

        Ok(true)
    }
}
