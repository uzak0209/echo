use uuid::Uuid;

use crate::{application::error::AppError, domain::repositories::PostRepository};
use std::sync::Arc;

pub struct IncrementDisplayCountUseCase {
    post_repository: Arc<dyn PostRepository>,
}

impl IncrementDisplayCountUseCase {
    pub fn new(post_repository: Arc<dyn PostRepository>) -> Self {
        Self { post_repository }
    }

    pub async fn execute(&self, post_id: Uuid) -> Result<bool, AppError> {
        // Check if post exists first
        if let Some(_post) = self.post_repository.find_by_id(post_id).await? {
            // Increment display count in repository
            // The repository will automatically set valid=false when display_count >= 10
            let _updated_post = self.post_repository.increment_display_count(post_id).await?;

            // Note: We no longer delete posts, just mark them as invalid (valid=false)
            // This allows for potential recovery or analytics

            Ok(true)
        } else {
            Ok(false)
        }
    }
}
