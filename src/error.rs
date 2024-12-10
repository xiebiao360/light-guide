use axum::{
    extract::multipart,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorOutput {
    pub error: String,
}

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Not found: {0}")]
    NotFound(String),

    #[error("sql error: {0}")]
    SqlxError(#[from] sqlx::Error),

    #[error("docker error: {0}")]
    DockerError(#[from] bollard::errors::Error),

    #[error("general error: {0}")]
    AnyError(#[from] anyhow::Error),

    #[error("migrate error: {0}")]
    MigrateError(#[from] sqlx::migrate::MigrateError),

    #[error("multipart error: {0}")]
    MultipartError(#[from] multipart::MultipartError),

    #[error("io error: {0}")]
    IOError(#[from] std::io::Error),
}

impl ErrorOutput {
    pub fn new(error: impl Into<String>) -> Self {
        Self {
            error: error.into(),
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response<axum::body::Body> {
        let status = match &self {
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::SqlxError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::DockerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::AnyError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::MigrateError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            AppError::MultipartError(_) => StatusCode::BAD_REQUEST,
            AppError::IOError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        };

        (status, Json(ErrorOutput::new(self.to_string()))).into_response()
    }
}
