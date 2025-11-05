use std::sync::Arc;
use crate::{
    domain::{
        entities::PostId,
        repositories::PostRepository,
    },
    application::error::AppError,
};

pub struct IncrementDisplayCountUseCase {
    post_repository: Arc<dyn PostRepository>,
}

impl IncrementDisplayCountUseCase {
    pub fn new(post_repository: Arc<dyn PostRepository>) -> Self {
        Self { post_repository }
    }

    pub async fn execute(&self, post_id: i32) -> Result<bool, AppError> {
        let post_id = PostId(post_id);

        if let Some(mut post) = self.post_repository.find_by_id(post_id).await? {
            post.increment_display();

            // If post is expired (display_count >= 10), delete it
            if post.is_expired() {
                self.post_repository.delete(post_id).await?;
            } else {
                self.post_repository.save(&post).await?;
            }

            Ok(true)
        } else {
            Ok(false)
        }
    }
}
