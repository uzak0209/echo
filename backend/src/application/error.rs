use thiserror::Error;
use crate::domain::error::{DomainError, ValidationError};

/// Application層の総合エラー型
/// 複数種類のエラー（DB, Validation, Infraなど）をまとめて管理
#[derive(Debug, Error)]
pub enum AppError {
    #[error("Validation error: {0}")]
    Validation(#[from] ValidationError),

    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),

    #[error("Database error: {0}")]
    Database(#[from] sea_orm::DbErr),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Unknown error: {0}")]
    Unknown(#[from] anyhow::Error),
}

impl AppError {
    /// Internal errorを作成するヘルパー
    pub fn internal(msg: impl Into<String>) -> Self {
        Self::Internal(msg.into())
    }

    /// Not found errorを作成するヘルパー
    pub fn not_found(msg: impl Into<String>) -> Self {
        Self::NotFound(msg.into())
    }
}

// AppErrorはDisplay + Send + Syncを実装しているため、
// async-graphqlが自動的にAsync_graphql::Errorに変換してくれます
