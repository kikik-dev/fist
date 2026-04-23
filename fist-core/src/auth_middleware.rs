use axum::{
    body::Body,
    extract::Request,
    middleware::Next,
    response::{IntoResponse, Response},
    http::StatusCode,
};
use crate::auth::verify_token;

pub async fn auth_middleware(req: Request<Body>, next: Next) -> Result<Response, Response> {
    let auth_header = req.headers().get("Authorization");
    
    let token = auth_header
        .and_then(|h| h.to_str().ok())
        .and_then(|s| s.strip_prefix("Bearer "))
        .ok_or((StatusCode::UNAUTHORIZED, "Missing or invalid token").into_response())?;

    // In a real app, load the secret from environment variables
    let secret = b"super-secret-key"; 
    
    verify_token(token, secret)
        .map_err(|_| (StatusCode::UNAUTHORIZED, "Invalid token").into_response())?;

    Ok(next.run(req).await)
}
