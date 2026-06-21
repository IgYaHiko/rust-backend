use std::sync::Arc;
use sqlx::PgPool;
use crate::auth::jwt::JwtService;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub jwt_service: Arc<JwtService>,
}
