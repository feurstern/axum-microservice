use axum::http::StatusCode;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ApiError {
    #[error("Database error : {0}")]
    Database(#[from] diesel::result::Error),
    #[error("Internal server error : {0}")]
    Internal(String),
}

impl From<ApiError> for (StatusCode, String) {
    fn from(err: ApiError) -> Self {
        match err {
            ApiError::Database(e) => (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()),
            ApiError::Internal(e) => (StatusCode::INTERNAL_SERVER_ERROR, e),
        }
    }
}
