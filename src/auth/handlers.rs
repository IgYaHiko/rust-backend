use axum::{
    extract::State,
    Json,
};
use bcrypt::{hash, verify, DEFAULT_COST};

use crate::models::{RegisterRequest, LoginRequest, AuthResponse, UserResponse};
use crate::error::{AppError, AppResult};
use crate::state::AppState;

pub async fn register(
    State(state): State<AppState>,
    Json(payload): Json<RegisterRequest>,
) -> AppResult<Json<AuthResponse>> {
    let pool = &state.pool;
    let jwt_service = &state.jwt_service;

    // Check if user already exists
    let exists = sqlx::query!(
        "SELECT id FROM users WHERE email = $1",
        payload.email
    )
    .fetch_optional(pool)
    .await?
    .is_some();

    if exists {
        return Err(AppError::UserExists);
    }

    let password_hash = hash(&payload.password, DEFAULT_COST)
        .map_err(|_| AppError::InternalError)?;

    let user = sqlx::query!(
        r#"
        INSERT INTO users (email, username, password_hash)
        VALUES ($1, $2, $3)
        RETURNING id, email, username, password_hash, created_at, updated_at
        "#,
        payload.email,
        payload.username,
        password_hash
    )
    .fetch_one(pool)
    .await?;

    let token = jwt_service
        .generate_token(&user.id, &user.email)
        .map_err(|_| AppError::InternalError)?;

    let response = AuthResponse {
        token,
        user: UserResponse {
            id: user.id,
            email: user.email,
            username: user.username,
            created_at: user.created_at.unwrap_or_else(chrono::Utc::now),
            updated_at: user.updated_at.unwrap_or_else(chrono::Utc::now),
        },
    };

    Ok(Json(response))
}

pub async fn login(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> AppResult<Json<AuthResponse>> {
    let pool = &state.pool;
    let jwt_service = &state.jwt_service;

    let user = sqlx::query!(
        r#"
        SELECT id, email, username, password_hash, created_at, updated_at
        FROM users
        WHERE email = $1
        "#,
        payload.email
    )
    .fetch_optional(pool)
    .await?
    .ok_or(AppError::InvalidCredentials)?;

    let is_valid = verify(&payload.password, &user.password_hash)
        .map_err(|_| AppError::InternalError)?;

    if !is_valid {
        return Err(AppError::InvalidCredentials);
    }

    let token = jwt_service
        .generate_token(&user.id, &user.email)
        .map_err(|_| AppError::InternalError)?;

    let response = AuthResponse {
        token,
        user: UserResponse {
            id: user.id,
            email: user.email,
            username: user.username,
            created_at: user.created_at.unwrap_or_else(chrono::Utc::now),
            updated_at: user.updated_at.unwrap_or_else(chrono::Utc::now),
        },
    };

    Ok(Json(response))
}
