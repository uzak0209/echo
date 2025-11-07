use std::sync::Arc;
use rand::seq::SliceRandom;
use uuid::Uuid;
use crate::{
    application::{dto::PostDto, error::AppError},
    domain::repositories::PostRepository,
};

pub struct GetTimelineUseCase {
    post_repository: Arc<dyn PostRepository>,
}

impl GetTimelineUseCase {
    pub fn new(
        post_repository: Arc<dyn PostRepository>,
    ) -> Self {
        Self {
            post_repository,
        }
    }

    pub async fn execute(&self, limit: usize, exclude_user_id: Option<Uuid>) -> Result<Vec<PostDto>, AppError> {
        // Get available posts with user data using JOIN (display_count < 10 and valid=true), excluding own posts
        let mut posts_with_users = self.post_repository.find_available_with_users(limit, exclude_user_id).await?;

        // Shuffle randomly (in a separate scope to drop rng before async operations)
        {
            let mut rng = rand::thread_rng();
            posts_with_users.shuffle(&mut rng);
        }

        // Convert to DTOs with user information and increment display count
        let mut dtos = Vec::new();
        for (post, user) in posts_with_users {
            // Increment display count for each post being viewed
            let updated_post = self.post_repository.increment_display_count(post.id).await?;

            // If post is expired (display_count >= 10), it will be filtered out next time
            // The post is already marked as invalid in the database by increment_display_count

            let dto = PostDto::new(
                updated_post,
                user.display_name.value().to_string(),
                user.avatar_url.clone(),
            );

            dtos.push(dto);
        }

        Ok(dtos)
    }
}
