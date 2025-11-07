use crate::{
    application::error::AppError,
    domain::repositories::UserRepository,
    infrastructure::auth::JwtService,
};
use bcrypt::verify;
use std::sync::Arc;

pub struct LoginUseCase {
    user_repository: Arc<dyn UserRepository>,
    jwt_service: Arc<JwtService>,
}

#[derive(Debug)]
pub struct LoginTokens {
    pub access_token: String,
    pub refresh_token: String,
    pub user_id: String,
}

impl LoginUseCase {
    pub fn new(user_repository: Arc<dyn UserRepository>, jwt_service: Arc<JwtService>) -> Self {
        Self {
            user_repository,
            jwt_service,
        }
    }

    pub async fn execute(&self, email: String, password: String) -> Result<LoginTokens, AppError> {
        // Find user by email
        let user = self
            .user_repository
            .find_by_email(&email)
            .await?
            .ok_or_else(|| AppError::not_found("User not found"))?;

        // Verify password
        let password_hash = user
            .password_hash
            .ok_or_else(|| AppError::validation("User does not have password authentication enabled"))?;

        let is_valid = verify(&password, &password_hash)
            .map_err(|e| AppError::internal(format!("Password verification failed: {}", e)))?;

        if !is_valid {
            return Err(AppError::validation("Invalid password"));
        }

        // Generate tokens
        let access_token = self
            .jwt_service
            .generate_access_token(user.id)
            .map_err(|e| AppError::internal(format!("Failed to generate access token: {}", e)))?;

        let refresh_token = self
            .jwt_service
            .generate_refresh_token(user.id)
            .map_err(|e| AppError::internal(format!("Failed to generate refresh token: {}", e)))?;

        Ok(LoginTokens {
            access_token,
            refresh_token,
            user_id: user.id.to_string(),
        })
    }
}
