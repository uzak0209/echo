use async_graphql::{Context, Object, Result};
use std::sync::Arc;
use uuid::Uuid;
use crate::application::usecases::GetTimelineUseCase;
use super::types::Post;

#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn timeline(&self, ctx: &Context<'_>, limit: i32, user_id: Option<String>) -> Result<Vec<Post>> {
        let use_case = ctx.data::<Arc<GetTimelineUseCase>>()?;

        // Parse user_id if provided to exclude own posts
        let exclude_user_id = user_id
            .and_then(|id| Uuid::parse_str(&id).ok());

        let posts = use_case.execute(limit as usize, exclude_user_id).await?;

        Ok(posts.into_iter().map(Post::from).collect())
    }
}
