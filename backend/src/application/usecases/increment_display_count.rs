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
            let _updated_post = self
                .post_repository
                .increment_display_count(post_id)
                .await?;

            Ok(true)
        } else {
            Ok(false)
        }
    }
}
