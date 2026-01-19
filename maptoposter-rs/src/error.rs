use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;

/// Application error types
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Geocoding failed: {0}")]
    Geocoding(String),

    #[error("Theme not found: {0}")]
    ThemeNotFound(String),

    #[error("Invalid distance: {0}")]
    InvalidDistance(String),

    #[error("Data fetch failed: {0}")]
    DataFetch(String),

    #[error("Rendering failed: {0}")]
    Rendering(String),

    #[error("Job not found: {0}")]
    JobNotFound(String),

    #[error("Internal error: {0}")]
    Internal(String),

    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON error: {0}")]
    Json(#[from] serde_json::Error),
}

/// Error response body
#[derive(Serialize)]
struct ErrorResponse {
    error: String,
    message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    details: Option<serde_json::Value>,
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, error_type, message) = match &self {
            AppError::Geocoding(msg) => (StatusCode::BAD_REQUEST, "geocoding_error", msg.clone()),
            AppError::ThemeNotFound(name) => (
                StatusCode::NOT_FOUND,
                "theme_not_found",
                format!("Theme '{}' not found", name),
            ),
            AppError::InvalidDistance(msg) => {
                (StatusCode::BAD_REQUEST, "invalid_distance", msg.clone())
            }
            AppError::DataFetch(msg) => {
                (StatusCode::SERVICE_UNAVAILABLE, "data_fetch_error", msg.clone())
            }
            AppError::Rendering(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "rendering_error", msg.clone())
            }
            AppError::JobNotFound(id) => (
                StatusCode::NOT_FOUND,
                "job_not_found",
                format!("Job '{}' not found", id),
            ),
            AppError::Internal(msg) => {
                (StatusCode::INTERNAL_SERVER_ERROR, "internal_error", msg.clone())
            }
            AppError::Request(e) => (
                StatusCode::SERVICE_UNAVAILABLE,
                "request_error",
                e.to_string(),
            ),
            AppError::Io(e) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "io_error",
                e.to_string(),
            ),
            AppError::Json(e) => (
                StatusCode::BAD_REQUEST,
                "json_error",
                e.to_string(),
            ),
        };

        let body = ErrorResponse {
            error: error_type.to_string(),
            message,
            details: None,
        };

        (status, Json(body)).into_response()
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
