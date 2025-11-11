use crate::{
    application::error::AppError,
    domain::{repositories::UserRepository, services::PersonaGenerator},
    infrastructure::auth::JwtService,
};
use bcrypt::{hash, DEFAULT_COST};
use std::sync::Arc;

pub struct SignupUseCase {
    user_repository: Arc<dyn UserRepository>,
    jwt_service: Arc<JwtService>,
}

#[derive(Debug)]
pub struct SignupTokens {
    pub access_token: String,
    pub refresh_token: String,
    pub user_id: String,
}

impl SignupUseCase {
    pub fn new(user_repository: Arc<dyn UserRepository>, jwt_service: Arc<JwtService>) -> Self {
        Self {
            user_repository,
            jwt_service,
        }
    }

    pub async fn execute(
        &self,
        username: String,
        password: String,
        avatar_url: Option<String>,
    ) -> Result<SignupTokens, AppError> {
        // Check if user already exists
        if (self.user_repository.find_by_username(&username).await?).is_some() {
            return Err(AppError::validation("Username already registered"));
        }

        // Hash password
        let password_hash = hash(password, DEFAULT_COST)
            .map_err(|e| AppError::internal(format!("Failed to hash password: {}", e)))?;

        // Generate random avatar if not provided
        let final_avatar_url = avatar_url.unwrap_or_else(PersonaGenerator::generate_avatar);

        // Create user
        let user = self
            .user_repository
            .create_user_with_credentials(username, Some(final_avatar_url), password_hash)
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

        // Save refresh token to database
        self.user_repository
            .update_refresh_token(user.id, Some(refresh_token.clone()))
            .await?;

        Ok(SignupTokens {
            access_token,
            refresh_token,
            user_id: user.id.to_string(),
        })
    }
}
