use crate::{
    application::error::AppError,
    infrastructure::auth::JwtService,
};
use std::sync::Arc;
use uuid::Uuid;

pub struct RefreshTokenUseCase {
    jwt_service: Arc<JwtService>,
}

pub struct RefreshedTokens {
    pub access_token: String,
}

impl RefreshTokenUseCase {
    pub fn new(jwt_service: Arc<JwtService>) -> Self {
        Self { jwt_service }
    }

    pub async fn execute(&self, refresh_token: &str) -> Result<RefreshedTokens, AppError> {
        // Verify refresh token
        let claims = self
            .jwt_service
            .verify_refresh_token(refresh_token)
            .map_err(|e| AppError::not_found(format!("Invalid refresh token: {}", e)))?;

        // Parse user_id from claims
        let user_id = Uuid::parse_str(&claims.sub)
            .map_err(|e| AppError::internal(format!("Invalid user_id in token: {}", e)))?;

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
