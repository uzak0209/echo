use async_graphql::{Context, Object, Result};
use std::sync::Arc;
use crate::application::usecases::{CreatePostUseCase, IncrementDisplayCountUseCase};

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

    async fn increment_display_count(&self, ctx: &Context<'_>, post_id: i32) -> Result<bool> {
        let use_case = ctx.data::<Arc<IncrementDisplayCountUseCase>>()?;

        use_case.execute(post_id).await?;

        Ok(true)
    }
}
