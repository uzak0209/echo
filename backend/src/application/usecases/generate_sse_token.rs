use crate::{application::error::AppError, infrastructure::auth::JwtService};
use std::sync::Arc;
use uuid::Uuid;

pub struct GenerateSseTokenUseCase {
    jwt_service: Arc<JwtService>,
}

impl GenerateSseTokenUseCase {
    pub fn new(jwt_service: Arc<JwtService>) -> Self {
        Self { jwt_service }
    }

    pub async fn execute(&self, user_id: Uuid) -> Result<String, AppError> {
        // Generate short-lived SSE token (60 seconds)
        let sse_token = self
            .jwt_service
            .generate_sse_token(user_id)
            .map_err(|e| AppError::internal(format!("Failed to generate SSE token: {}", e)))?;

        Ok(sse_token)
    }
}
