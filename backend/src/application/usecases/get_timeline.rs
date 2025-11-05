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
        // Get available posts (display_count < 10), excluding own posts
        let mut posts = self.post_repository.find_available(limit, exclude_user_id).await?;

        // Shuffle randomly
        let mut rng = rand::thread_rng();
        posts.shuffle(&mut rng);

        // Convert to DTOs with user information
        let mut dtos = Vec::new();
        for post in posts {
            let user = self.user_repository.find_by_id(post.user_id).await?;

            let dto = if let Some(user) = user {
                PostDto::new(
                    post,
                    user.display_name.value().to_string(),
                    user.avatar_url.clone(),
                )
            } else {
                PostDto::from(post)
            };

            dtos.push(dto);
        }

        Ok(dtos)
    }
}
