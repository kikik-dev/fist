use axum::{body::Body, middleware::Next, response::Response};
use tracing::info;

pub async fn logging_middleware(request: axum::http::Request<Body>, next: Next) -> Response {
    info!("Handling request: {} {}", request.method(), request.uri().path());
    next.run(request).await
}
