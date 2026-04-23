use axum::{
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;

pub enum FistError {
    Internal(String),
    Validation(String),
    NotFound(String),
}

impl IntoResponse for FistError {
    fn into_response(self) -> Response {
        let (status, message) = match self {
            FistError::Internal(msg) => (StatusCode::INTERNAL_SERVER_ERROR, msg),
            FistError::Validation(msg) => (StatusCode::BAD_REQUEST, msg),
            FistError::NotFound(msg) => (StatusCode::NOT_FOUND, msg),
        };

        (status, Json(json!({ "error": message }))).into_response()
    }
}
