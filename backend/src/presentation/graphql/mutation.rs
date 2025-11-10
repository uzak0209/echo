use crate::application::usecases::{
    AddReactionUseCase, CreatePostUseCase, GenerateSseTokenUseCase, IncrementDisplayCountUseCase,
    LoginUseCase, LogoutUseCase, RefreshTokenUseCase, RemoveReactionUseCase, SignupUseCase,
};
use crate::presentation::graphql::types::{AuthResponse, CreatePostInput, ReactionTypeGql, RefreshResponse};
use async_graphql::{Context, Object, Result};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Default)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn signup(
        &self,
        ctx: &Context<'_>,
        username: String,
        password: String,
        avatar_url: Option<String>,
    ) -> Result<AuthResponse> {
        let use_case = ctx.data::<Arc<SignupUseCase>>()?;

        let tokens = use_case.execute(username, password, avatar_url).await?;

        // Store refresh token in context for HTTP layer to set as cookie
        ctx.insert_http_header("X-Refresh-Token", tokens.refresh_token.clone());

        Ok(tokens.into())
    }

    async fn login(
        &self,
        ctx: &Context<'_>,
        username: String,
        password: String,
    ) -> Result<AuthResponse> {
        let use_case = ctx.data::<Arc<LoginUseCase>>()?;

        let tokens = use_case.execute(username, password).await?;

        // Store refresh token in context for HTTP layer to set as cookie
        ctx.insert_http_header("X-Refresh-Token", tokens.refresh_token.clone());

        Ok(tokens.into())
    }

    async fn refresh_token(&self, ctx: &Context<'_>) -> Result<RefreshResponse> {
        let use_case = ctx.data::<Arc<RefreshTokenUseCase>>()?;

        // Get refresh token from context (set by HTTP layer from cookie)
        let refresh_token = ctx
            .data::<String>()
            .map_err(|_| async_graphql::Error::new("Refresh token not found"))?;

        let refreshed_tokens = use_case.execute(refresh_token).await?;

        Ok(refreshed_tokens.into())
    }

    async fn logout(&self, ctx: &Context<'_>) -> Result<bool> {
        let use_case = ctx.data::<Arc<LogoutUseCase>>()?;

        // Get user_id from JWT context
        let user_id = ctx.data::<Uuid>()
            .map_err(|_| async_graphql::Error::new("Unauthorized: No valid access token"))?;

        use_case.execute(*user_id).await?;

        // Signal to HTTP layer to clear refresh token cookie
        ctx.insert_http_header("X-Clear-Refresh-Token", "true");

        Ok(true)
    }

    async fn create_post(
        &self,
        ctx: &Context<'_>,
        input: CreatePostInput,
    ) -> Result<bool> {
        let use_case = ctx.data::<Arc<CreatePostUseCase>>()?;

        // Get user_id from JWT context
        let user_id = ctx.data::<Uuid>()
            .map_err(|_| async_graphql::Error::new("Unauthorized: No valid access token"))?;

        use_case.execute(input.content, input.image_url, *user_id).await?;

        Ok(true)
    }

    async fn increment_display_count(&self, ctx: &Context<'_>, post_id: String) -> Result<bool> {
        let use_case = ctx.data::<Arc<IncrementDisplayCountUseCase>>()?;

        // Parse incoming string to UUID before handing to the application layer
        let post_uuid = Uuid::parse_str(&post_id)
            .map_err(|e| async_graphql::Error::new(format!("Invalid UUID: {}", e)))?;

        use_case.execute(post_uuid).await?;

        Ok(true)
    }

    async fn add_reaction(
        &self,
        ctx: &Context<'_>,
        post_id: String,
        reaction_type: ReactionTypeGql,
    ) -> Result<bool> {
        let use_case = ctx.data::<Arc<AddReactionUseCase>>()?;

        // Get user_id from JWT context
        let user_id = ctx.data::<Uuid>()
            .map_err(|_| async_graphql::Error::new("Unauthorized: No valid access token"))?;

        let post_uuid = Uuid::parse_str(&post_id)
            .map_err(|e| async_graphql::Error::new(format!("Invalid post UUID: {}", e)))?;

        use_case
            .execute(post_uuid, *user_id, reaction_type.into())
            .await?;

        Ok(true)
    }

    async fn remove_reaction(
        &self,
        ctx: &Context<'_>,
        post_id: String,
    ) -> Result<bool> {
        let use_case = ctx.data::<Arc<RemoveReactionUseCase>>()?;

        // Get user_id from JWT context
        let user_id = ctx.data::<Uuid>()
            .map_err(|_| async_graphql::Error::new("Unauthorized: No valid access token"))?;

        let post_uuid = Uuid::parse_str(&post_id)
            .map_err(|e| async_graphql::Error::new(format!("Invalid post UUID: {}", e)))?;

        use_case
            .execute(post_uuid, *user_id)
            .await?;

        Ok(true)
    }

    async fn generate_sse_token(&self, ctx: &Context<'_>) -> Result<String> {
        let use_case = ctx.data::<Arc<GenerateSseTokenUseCase>>()?;

        // Get user_id from JWT context
        let user_id = ctx.data::<Uuid>()
            .map_err(|_| async_graphql::Error::new("Unauthorized: No valid access token"))?;

        let sse_token = use_case.execute(*user_id).await?;

        Ok(sse_token)
    }
}
