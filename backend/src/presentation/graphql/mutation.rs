use crate::application::usecases::{CreatePostUseCase, IncrementDisplayCountUseCase};
use async_graphql::{Context, Object, Result};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Default)]
pub struct MutationRoot;

#[Object]
impl MutationRoot {
    async fn create_post(
        &self,
        ctx: &Context<'_>,
        content: String,
        image_url: Option<String>,
    ) -> Result<bool> {
        let use_case = ctx.data::<Arc<CreatePostUseCase>>()?;

        use_case.execute(content, image_url).await?;

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
}
