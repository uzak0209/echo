use async_graphql::{Context, Object, Result};
use std::sync::Arc;
use crate::application::usecases::GetTimelineUseCase;
use super::types::Post;

#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn timeline(&self, ctx: &Context<'_>, limit: i32) -> Result<Vec<Post>> {
        let use_case = ctx.data::<Arc<GetTimelineUseCase>>()?;

        let posts = use_case.execute(limit as usize).await?;

        Ok(posts.into_iter().map(Post::from).collect())
    }
}
