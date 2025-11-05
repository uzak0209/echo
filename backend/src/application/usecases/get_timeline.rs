use std::sync::Arc;
use rand::seq::SliceRandom;
use crate::{
    application::dto::PostDto,
    domain::repositories::PostRepository,
};

pub struct GetTimelineUseCase {
    post_repository: Arc<dyn PostRepository>,
}

impl GetTimelineUseCase {
    pub fn new(post_repository: Arc<dyn PostRepository>) -> Self {
        Self { post_repository }
    }

    pub async fn execute(&self, limit: usize) -> Result<Vec<PostDto>, Box<dyn std::error::Error + Send + Sync>> {
        // Get available posts (display_count < 10)
        let mut posts = self.post_repository.find_available(limit).await?;

        // Shuffle randomly
        let mut rng = rand::thread_rng();
        posts.shuffle(&mut rng);

        // Convert to DTOs
        let dtos: Vec<PostDto> = posts.into_iter().map(PostDto::from).collect();

        Ok(dtos)
    }
}
