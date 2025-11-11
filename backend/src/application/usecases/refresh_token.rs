use crate::{
    application::error::AppError,
    domain::repositories::UserRepository,
    infrastructure::auth::JwtService,
};
use std::sync::Arc;
use uuid::Uuid;

pub struct RefreshTokenUseCase {
    user_repository: Arc<dyn UserRepository>,
    jwt_service: Arc<JwtService>,
}

pub struct RefreshedTokens {
    pub access_token: String,
}

impl RefreshTokenUseCase {
    pub fn new(user_repository: Arc<dyn UserRepository>, jwt_service: Arc<JwtService>) -> Self {
        Self {
            user_repository,
            jwt_service,
        }
    }

    pub async fn execute(&self, refresh_token: &str) -> Result<RefreshedTokens, AppError> {
        // Verify refresh token signature
        let claims = self
            .jwt_service
            .verify_refresh_token(refresh_token)
            .map_err(|e| AppError::not_found(format!("Invalid refresh token: {}", e)))?;

        // Parse user_id from claims
        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|e| AppError::internal(format!("Invalid user_id in token: {}", e)))?;

        // Verify token exists in database
        let user = self
            .user_repository
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| AppError::not_found("User not found"))?;

        // Check if the refresh token matches the one stored in DB
        let stored_token = user
            .refresh_token
            .ok_or_else(|| AppError::validation("No refresh token found for this user"))?;

        if stored_token != refresh_token {
            return Err(AppError::validation("Invalid refresh token"));
        }

        // Generate new access token only
        let new_access_token = self
            .jwt_service
            .generate_access_token(user_id)
            .map_err(|e| AppError::internal(format!("Failed to generate access token: {}", e)))?;

        Ok(RefreshedTokens {
            access_token: new_access_token,
        })
    }
}
