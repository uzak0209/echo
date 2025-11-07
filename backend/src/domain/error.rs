use thiserror::Error;

/// ドメイン層のバリデーションエラー
#[derive(Debug, Error)]
pub enum ValidationError {
    #[error("Content cannot be empty")]
    EmptyContent,

    #[error("Content too long (max {max} characters, got {actual})")]
    ContentTooLong { max: usize, actual: usize },

    #[error("Invalid input: {0}")]
    InvalidInput(String),
}

impl ValidationError {
    pub fn new(msg: String) -> Self {
        Self::InvalidInput(msg)
    }
}

/// ドメイン層の一般的なエラー
#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),

    #[error("Database error: {0}")]
    Database(#[from] sea_orm::DbErr),

    #[error("Entity not found: {0}")]
    NotFound(String),

    #[error("Business rule violation: {0}")]
    BusinessRuleViolation(String),

    #[error("Unknown domain error: {0}")]
    Unknown(String),
}

impl DomainError {
    pub fn validation(msg: String) -> Self {
        Self::Validation(ValidationError::new(msg))
    }
}
