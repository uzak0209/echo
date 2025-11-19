use crate::{
    application::error::AppError,
    domain::{
        entities::Post,
        repositories::{PostRepository, UserRepository},
        value_objects::PostContent,
    },
    infrastructure::sse::{post_stream::PostEvent, PostStreamManager},
};
use std::sync::Arc;

pub struct CreatePostUseCase {
    post_repository: Arc<dyn PostRepository>,
    user_repository: Arc<dyn UserRepository>,
    post_stream_manager: Arc<PostStreamManager>,
}

impl CreatePostUseCase {
    pub fn new(
        post_repository: Arc<dyn PostRepository>,
        user_repository: Arc<dyn UserRepository>,
        post_stream_manager: Arc<PostStreamManager>,
    ) -> Self {
        Self {
            post_repository,
            user_repository,
            post_stream_manager,
        }
    }

    pub async fn execute(
        &self,
        content: String,
        image_url: Option<String>,
        user_id: uuid::Uuid,
    ) -> Result<bool, AppError> {
        // Validate content
        let post_content = PostContent::new(content.clone())?;

        // Verify user exists
        let user = match self.user_repository.find_by_id(user_id).await? {
            Some(user) => user,
            None => {
                return Err(AppError::not_found("User not found"));
            }
        };

        // Create new post
        let post = Post::new(user.id, post_content, image_url.clone());

        self.post_repository.create(&post).await?;

        // Broadcast new post event to all connected clients
        let event = PostEvent::new_post(
            post.id,
            post.user_id,
            content,
            image_url,
            post.display_count.value(),
            post.created_at,
            user.display_name.value().to_string(),
            user.avatar_url,
        );

        // Fire and forget - don't fail the mutation if broadcasting fails
        let _ = self.post_stream_manager.broadcast(event).await;

        Ok(true)
    }
}
