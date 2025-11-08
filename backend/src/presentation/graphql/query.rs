use super::types::{Post, ReactionTypeGql, UserExpressionState};
use crate::application::usecases::{GetTimelineUseCase, GetUserExpressionStateUseCase, GetUserLatestReactionUseCase};
use async_graphql::{Context, Object, Result};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Default)]
pub struct QueryRoot;

#[Object]
impl QueryRoot {
    async fn timeline(
        &self,
        ctx: &Context<'_>,
        limit: i32,
    ) -> Result<Vec<Post>> {
        let use_case = ctx.data::<Arc<GetTimelineUseCase>>()?;

        // Get user_id from JWT context to automatically exclude own posts
        let user_id = ctx.data::<Uuid>()
            .map_err(|_| async_graphql::Error::new("Unauthorized: No valid access token"))?;

        let posts = use_case.execute(limit as usize, Some(*user_id)).await?;

        Ok(posts.into_iter().map(Post::from).collect())
    }

    async fn user_latest_reaction(
        &self,
        ctx: &Context<'_>,
        user_id: String,
    ) -> Result<Option<ReactionTypeGql>> {
        let use_case = ctx.data::<Arc<GetUserLatestReactionUseCase>>()?;

        let user_uuid = Uuid::parse_str(&user_id)
            .map_err(|e| async_graphql::Error::new(format!("Invalid UUID: {}", e)))?;

        let reaction_type = use_case.execute(user_uuid).await?;

        Ok(reaction_type.map(|r| r.into()))
    }

    async fn user_expression_state(&self, ctx: &Context<'_>) -> Result<UserExpressionState> {
        let use_case = ctx.data::<Arc<GetUserExpressionStateUseCase>>()?;

        // Get user_id from JWT context
        let user_id = ctx.data::<Uuid>()
            .map_err(|_| async_graphql::Error::new("Unauthorized: No valid access token"))?;

        let expression_state = use_case.execute(*user_id).await?;

        Ok(expression_state.into())
    }
}
