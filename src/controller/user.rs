use axum::{extract::State, Json};
use crate::dto::user::CreateUserDto;
use crate::service::user::UserService;
use fist_core::state::SharedState;
use fist_core::error::FistError;
use fist_core::request::FistRequest;

pub async fn register_user(
    State(state): State<SharedState>,
    Json(payload): Json<CreateUserDto>,
) -> Result<Json<serde_json::Value>, FistError> {
    // 1. Validate DTO
    payload.validate_request()?;

    // 2. Call Service
    let service = UserService { db: state.db.clone() };
    service.create_user(&payload.username, &payload.email).await?;

    Ok(Json(serde_json::json!({"status": "user created"})))
}
