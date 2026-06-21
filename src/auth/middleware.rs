use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;

use crate::error::AppError;
use crate::auth::jwt::JwtService;
use crate::models::AuthUser;
use uuid::Uuid;

pub async fn auth_middleware(
    State(jwt_service): State<Arc<JwtService>>,
    mut req: Request,
    next: Next,
) -> Result<Response, AppError> {
    // Extract token from Authorization header
    let auth_header = req
        .headers()
        .get("Authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| AppError::Unauthorized)?;

    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::Unauthorized);
    }

    let token = &auth_header[7..];

    let claims = jwt_service
        .verify_token(token)
        .map_err(|_| AppError::Unauthorized)?;

    let user_id = Uuid::parse_str(&claims.sub)
        .map_err(|_| AppError::Unauthorized)?;

    let auth_user = AuthUser {
        id: user_id,
        email: claims.email,
        username: "".to_string(),
    };

    req.extensions_mut().insert(auth_user);

    Ok(next.run(req).await)
}
