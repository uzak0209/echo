use crate::{
    application::error::AppError,
    domain::{
        entities::Post,
        repositories::{PostRepository, UserRepository},
        value_objects::PostContent,
    },
};
use std::sync::Arc;

pub struct CreatePostUseCase {
    post_repository: Arc<dyn PostRepository>,
    user_repository: Arc<dyn UserRepository>,
}

impl CreatePostUseCase {
    pub fn new(
        post_repository: Arc<dyn PostRepository>,
        user_repository: Arc<dyn UserRepository>,
    ) -> Self {
        Self {
            post_repository,
            user_repository,
        }
    }

    pub async fn execute(
        &self,
        content: String,
        image_url: Option<String>,
    ) -> Result<bool, AppError> {
        // Validate content
        let post_content = PostContent::new(content)?;

        // Get or create random user
        let user = match self.user_repository.get_random().await? {
            Some(user) => user,
            None => {
                let new_user = RandomUserService::generate_random_user();
                let user_id = self.user_repository.save(&new_user).await?;
                self.user_repository
                    .find_by_id(user_id)
                    .await?
                    .ok_or_else(|| AppError::internal("Failed to create user"))?
            }
        };

        // Create new post
        let post = Post::new(user.id, post_content, image_url);

        self.post_repository.save(&post).await?;

        Ok(true)
    }
}
