use crate::{
    application::error::AppError,
    domain::repositories::UserRepository,
};
use std::sync::Arc;
use uuid::Uuid;

pub struct LogoutUseCase {
    user_repository: Arc<dyn UserRepository>,
}

impl LogoutUseCase {
    pub fn new(user_repository: Arc<dyn UserRepository>) -> Self {
        Self { user_repository }
    }

    pub async fn execute(&self, user_id: Uuid) -> Result<(), AppError> {
        // Remove refresh token from database
        self.user_repository
            .update_refresh_token(user_id, None)
            .await?;

        Ok(())
    }
}
