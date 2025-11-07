use crate::{
    application::{dto::PostDto, error::AppError},
    domain::repositories::{PostRepository, ReactionRepository, UserRepository},
};
use std::sync::Arc;
use uuid::Uuid;

pub struct GetMyPostsUseCase {
    post_repository: Arc<dyn PostRepository>,
    user_repository: Arc<dyn UserRepository>,
    reaction_repository: Arc<dyn ReactionRepository>,
}

impl GetMyPostsUseCase {
    pub fn new(
        post_repository: Arc<dyn PostRepository>,
        user_repository: Arc<dyn UserRepository>,
        reaction_repository: Arc<dyn ReactionRepository>,
    ) -> Self {
        Self {
            post_repository,
            user_repository,
            reaction_repository,
        }
    }

    pub async fn execute(&self, user_id: Uuid) -> Result<Vec<PostDto>, AppError> {
        // Get all valid posts by this user
        let posts = self.post_repository.find_by_user_id(user_id).await?;

        // Get user info
        let user = self
            .user_repository
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::not_found("User not found"))?;

        // Convert to DTOs with reaction info
        let mut dtos = Vec::new();
        for post in posts {
            let dto = PostDto::new(
                post,
                user.display_name.value().to_string(),
                user.avatar_url.clone(),
            );
            dtos.push(dto);
        }

        Ok(dtos)
    }
}
