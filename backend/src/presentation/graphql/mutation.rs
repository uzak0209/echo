use crate::application::usecases::{
    CreatePostUseCase, CreateUserUseCase, EchoPostUseCase, IncrementDisplayCountUseCase, RefreshTokenUseCase,
};
use crate::presentation::graphql::types::{AuthResponse, RefreshResponse};
use async_graphql::{Context, Object, Result};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Default)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_user(
        &self,
        ctx: &Context<'_>,
        display_name: String,
        avatar_url: Option<String>,
    ) -> Result<AuthResponse> {
        let use_case = ctx.data::<Arc<CreateUserUseCase>>()?;

        let auth_tokens = use_case.execute(display_name, avatar_url).await?;

        // Store refresh token in context for HTTP layer to set as cookie
        ctx.insert_http_header("X-Refresh-Token", auth_tokens.refresh_token.clone());

        Ok(auth_tokens.into())
    }

    async fn refresh_token(&self, ctx: &Context<'_>) -> Result<RefreshResponse> {
        let use_case = ctx.data::<Arc<RefreshTokenUseCase>>()?;

        // Get refresh token from context (set by HTTP layer from cookie)
        let refresh_token = ctx
            .data::<String>()
            .map_err(|_| async_graphql::Error::new("Refresh token not found"))?;

        let refreshed_tokens = use_case.execute(refresh_token).await?;

        // Store new refresh token in context for HTTP layer to set as cookie
        ctx.insert_http_header("X-Refresh-Token", refreshed_tokens.refresh_token.clone());

        Ok(refreshed_tokens.into())
    }

    async fn create_post(
        &self,
        ctx: &Context<'_>,
        content: String,
        image_url: Option<String>,
        user_id: String,
    ) -> Result<bool> {
        let use_case = ctx.data::<Arc<CreatePostUseCase>>()?;

        // Parse incoming string to UUID before handing to the application layer
        let user_uuid = Uuid::parse_str(&user_id)
            .map_err(|e| async_graphql::Error::new(format!("Invalid UUID: {}", e)))?;

        use_case.execute(content, image_url, user_uuid).await?;

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

    async fn echo_post(&self, ctx: &Context<'_>, post_id: String) -> Result<bool> {
        let use_case = ctx.data::<Arc<EchoPostUseCase>>()?;

        // Parse incoming string to UUID before handing to the application layer
        let post_uuid = Uuid::parse_str(&post_id)
            .map_err(|e| async_graphql::Error::new(format!("Invalid UUID: {}", e)))?;

        use_case.execute(post_uuid).await?;

        Ok(true)
    }
}
