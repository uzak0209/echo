use uuid::Uuid;

use crate::{application::error::AppError, domain::repositories::PostRepository};
use std::sync::Arc;

pub struct EchoPostUseCase {
    post_repository: Arc<dyn PostRepository>,
}

impl EchoPostUseCase {
    pub fn new(post_repository: Arc<dyn PostRepository>) -> Self {
        Self { post_repository }
    }

    pub async fn execute(&self, post_id: Uuid) -> Result<bool, AppError> {
        // Increment echo count (silent resonance)
        self.post_repository.increment_echo_count(post_id).await?;
        Ok(true)
    }
}
