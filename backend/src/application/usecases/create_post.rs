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
        user_id: uuid::Uuid,
    ) -> Result<bool, AppError> {
        // Validate content
        let post_content = PostContent::new(content)?;

        // Verify user exists
        let user = match self.user_repository.find_by_id(user_id).await? {
            Some(user) => user,
            None => {
                return Err(AppError::not_found("User not found"));
            }
        };

        // Create new post
        let post = Post::new(user.id, post_content, image_url);

        self.post_repository.create(&post).await?;

        Ok(true)
    }
}
