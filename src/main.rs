use axum::{routing::{get, post}, Json, middleware};
use fist_core::router::FistRouter;
use fist_core::error::FistError;
use fist_core::middleware::logging_middleware;
use fist_core::state::{AppState, SharedState};
use fist_core::db::DbPool;
use std::sync::Arc;

#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

mod controller;
mod service;
mod dto;
mod model;
mod config;

async fn hello_world_handler() -> Result<Json<serde_json::Value>, FistError> {
    Ok(Json(serde_json::json!({"message": "One Punch!"})))
}

#[tokio::main]
async fn main() {
    // Load configuration
    let cfg = config::Config::from_env();
    
    // Create DB Pool
    let db = DbPool::connect(&cfg.database_url).await.expect("Failed to connect to DB");
    let state = Arc::new(AppState { db });

    let app = FistRouter::<SharedState>::new()
        .route("/", get(hello_world_handler))
        .route("/users", post(controller::user::register_user))
        .build()
        .with_state(state)
        .layer(middleware::from_fn(logging_middleware));

    let addr = std::net::SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Fist framework listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
