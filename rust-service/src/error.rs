use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

#[derive(Debug, thiserror::Error)]
pub enum AppError {
    #[error("student not found")]
    NotFound,
    #[error("upstream API error: {0}")]
    Upstream(String),
    #[error("authentication with Node API failed: {0}")]
    Auth(String),
    #[error("request failed: {0}")]
    Request(#[from] reqwest::Error),
    #[error("pdf generation failed: {0}")]
    Pdf(String),
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let (status, message) = match &self {
            AppError::NotFound => (StatusCode::NOT_FOUND, self.to_string()),
            AppError::Auth(_) => (StatusCode::BAD_GATEWAY, self.to_string()),
            AppError::Pdf(_) => (StatusCode::INTERNAL_SERVER_ERROR, self.to_string()),
            AppError::Upstream(_) | AppError::Request(_) => {
                (StatusCode::BAD_GATEWAY, self.to_string())
            }
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}

pub type AppResult<T> = Result<T, AppError>;
