use crate::{
    application::error::AppError,
    domain::repositories::UserRepository,
    infrastructure::auth::JwtService,
};
use std::sync::Arc;

pub struct CreateUserUseCase {
    user_repository: Arc<dyn UserRepository>,
    jwt_service: Arc<JwtService>,
}

#[derive(Debug)]
pub struct AuthTokens {
    pub access_token: String,
    pub refresh_token: String,
    pub user_id: String,
}

impl CreateUserUseCase {
    pub fn new(user_repository: Arc<dyn UserRepository>, jwt_service: Arc<JwtService>) -> Self {
        Self {
            user_repository,
            jwt_service,
        }
    }

    pub async fn execute(
        &self,
        display_name: String,
        avatar_url: Option<String>,
    ) -> Result<AuthTokens, AppError> {
        // Create new user
        let user = self
            .user_repository
            .create_user(display_name, avatar_url)
            .await?;

        // Generate tokens
        let access_token = self
            .jwt_service
            .generate_access_token(user.id)
            .map_err(|e| AppError::internal(format!("Failed to generate access token: {}", e)))?;

        let refresh_token = self
            .jwt_service
            .generate_refresh_token(user.id)
            .map_err(|e| AppError::internal(format!("Failed to generate refresh token: {}", e)))?;

        Ok(AuthTokens {
            access_token,
            refresh_token,
            user_id: user.id.to_string(),
        })
    }
}
