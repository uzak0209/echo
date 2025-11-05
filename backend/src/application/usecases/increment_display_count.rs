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
        if let Some(post) = self.post_repository.find_by_id(post_id).await? {
            // Increment display count in repository
            let updated_post = self.post_repository.increment_display_count(post_id).await?;

            // If post is expired (display_count >= 10), delete it
            if updated_post.is_expired() {
                self.post_repository.delete(post_id).await?;
            }

            Ok(true)
        } else {
            Ok(false)
        }
    }
}
