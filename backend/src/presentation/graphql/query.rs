use async_graphql::{Context, Object, Result};
use std::sync::Arc;
use uuid::Uuid;
use crate::application::usecases::{GetTimelineUseCase, GetMyPostsUseCase, GetPostReactionsUseCase, GetUserLatestReactionUseCase};
use super::types::{Post, ReactionCountGql, ReactionTypeGql};

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

    async fn my_posts(&self, ctx: &Context<'_>, user_id: String) -> Result<Vec<Post>> {
        let use_case = ctx.data::<Arc<GetMyPostsUseCase>>()?;

        let user_uuid = Uuid::parse_str(&user_id)
            .map_err(|e| async_graphql::Error::new(format!("Invalid UUID: {}", e)))?;

        let posts = use_case.execute(user_uuid).await?;

        Ok(posts.into_iter().map(Post::from).collect())
    }

    async fn post_reactions(&self, ctx: &Context<'_>, post_id: String) -> Result<Vec<ReactionCountGql>> {
        let use_case = ctx.data::<Arc<GetPostReactionsUseCase>>()?;

        let post_uuid = Uuid::parse_str(&post_id)
            .map_err(|e| async_graphql::Error::new(format!("Invalid UUID: {}", e)))?;

        let reactions = use_case.execute(post_uuid).await?;

        Ok(reactions.into_iter().map(ReactionCountGql::from).collect())
    }

    async fn user_latest_reaction(&self, ctx: &Context<'_>, user_id: String) -> Result<Option<ReactionTypeGql>> {
        let use_case = ctx.data::<Arc<GetUserLatestReactionUseCase>>()?;

        let user_uuid = Uuid::parse_str(&user_id)
            .map_err(|e| async_graphql::Error::new(format!("Invalid UUID: {}", e)))?;

        let reaction_type = use_case.execute(user_uuid).await?;

        Ok(reaction_type.map(|r| r.into()))
    }
}
