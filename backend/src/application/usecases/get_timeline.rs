use std::sync::Arc;
use rand::seq::SliceRandom;
use uuid::Uuid;
use crate::{
    application::{dto::PostDto, error::AppError},
    domain::repositories::{PostRepository, UserRepository},
};

pub struct GetTimelineUseCase {
    post_repository: Arc<dyn PostRepository>,
    user_repository: Arc<dyn UserRepository>,
}

impl GetTimelineUseCase {
    pub fn new(
        post_repository: Arc<dyn PostRepository>,
        user_repository: Arc<dyn UserRepository>,
    ) -> Self {
        Self {
            post_repository,
            user_repository,
        }
    }

    pub async fn execute(&self, limit: usize, exclude_user_id: Option<Uuid>) -> Result<Vec<PostDto>, AppError> {
        // Get available posts (display_count < 10 and valid=true), excluding own posts
        let mut posts = self.post_repository.find_available(limit, exclude_user_id).await?;

        // Shuffle randomly (in a separate scope to drop rng before async operations)
        {
            let mut rng = rand::thread_rng();
            posts.shuffle(&mut rng);
        }

        // Convert to DTOs with user information and increment display count
        let mut dtos = Vec::new();
        for post in posts {
            // Increment display count for each post being viewed
            let updated_post = self.post_repository.increment_display_count(post.id).await?;

            // If post is expired (display_count >= 10), it will be filtered out next time
            // The post is already marked as invalid in the database by increment_display_count

            let user = self.user_repository.find_by_id(updated_post.user_id).await?;

            let dto = if let Some(user) = user {
                PostDto::new(
                    updated_post,
                    user.display_name.value().to_string(),
                    user.avatar_url.clone(),
                )
            } else {
                PostDto::from(updated_post)
            };

            dtos.push(dto);
        }

        Ok(dtos)
    }
}
