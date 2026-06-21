use axum::{
    extract::{Path, State, Extension},
    Json,
};
use uuid::Uuid;

use crate::models::{CreatePostRequest, UpdatePostRequest, PostResponse, AuthUser};
use crate::error::{AppError, AppResult};
use crate::state::AppState;

pub async fn get_all(
    Extension(auth_user): Extension<AuthUser>,
    State(state): State<AppState>,
) -> AppResult<Json<Vec<PostResponse>>> {
    let pool = &state.pool;

    let posts = sqlx::query!(
        r#"
        SELECT id, title, content, user_id, created_at, updated_at
        FROM posts
        WHERE user_id = $1
        ORDER BY created_at DESC
        "#,
        auth_user.id
    )
    .fetch_all(pool)
    .await?
    .into_iter()
    .map(|post| PostResponse {
        id: post.id,
        title: post.title,
        content: post.content,
        user_id: post.user_id,
        created_at: post.created_at.unwrap_or_else(chrono::Utc::now),
        updated_at: post.updated_at.unwrap_or_else(chrono::Utc::now),
    })
    .collect();

    Ok(Json(posts))
}

pub async fn get_one(
    Extension(auth_user): Extension<AuthUser>,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<Json<PostResponse>> {
    let pool = &state.pool;

    let post = sqlx::query!(
        r#"
        SELECT id, title, content, user_id, created_at, updated_at
        FROM posts
        WHERE id = $1 AND user_id = $2
        "#,
        id,
        auth_user.id
    )
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Post not found".to_string()))?;

    let response = PostResponse {
        id: post.id,
        title: post.title,
        content: post.content,
        user_id: post.user_id,
        created_at: post.created_at.unwrap_or_else(chrono::Utc::now),
        updated_at: post.updated_at.unwrap_or_else(chrono::Utc::now),
    };

    Ok(Json(response))
}

pub async fn create(
    Extension(auth_user): Extension<AuthUser>,
    State(state): State<AppState>,
    Json(payload): Json<CreatePostRequest>,
) -> AppResult<Json<PostResponse>> {
    let pool = &state.pool;

    let post = sqlx::query!(
        r#"
        INSERT INTO posts (title, content, user_id)
        VALUES ($1, $2, $3)
        RETURNING id, title, content, user_id, created_at, updated_at
        "#,
        payload.title,
        payload.content,
        auth_user.id
    )
    .fetch_one(pool)
    .await?;

    let response = PostResponse {
        id: post.id,
        title: post.title,
        content: post.content,
        user_id: post.user_id,
        created_at: post.created_at.unwrap_or_else(chrono::Utc::now),
        updated_at: post.updated_at.unwrap_or_else(chrono::Utc::now),
    };

    Ok(Json(response))
}

pub async fn update(
    Extension(auth_user): Extension<AuthUser>,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdatePostRequest>,
) -> AppResult<Json<PostResponse>> {
    let pool = &state.pool;

    // Check if post exists and belongs to user
    let _existing = sqlx::query!(
        "SELECT id FROM posts WHERE id = $1 AND user_id = $2",
        id,
        auth_user.id
    )
    .fetch_optional(pool)
    .await?
    .ok_or_else(|| AppError::NotFound("Post not found".to_string()))?;

    // Simple approach: always update both fields, using existing values if not provided
    // First, get the current post
    let current = sqlx::query!(
        r#"
        SELECT title, content FROM posts WHERE id = $1
        "#,
        id
    )
    .fetch_one(pool)
    .await?;

    let new_title = payload.title.unwrap_or(current.title);
    let new_content = payload.content.unwrap_or(current.content);

    // Update with the new values
    let post = sqlx::query!(
        r#"
        UPDATE posts
        SET title = $1, content = $2, updated_at = CURRENT_TIMESTAMP
        WHERE id = $3 AND user_id = $4
        RETURNING id, title, content, user_id, created_at, updated_at
        "#,
        new_title,
        new_content,
        id,
        auth_user.id
    )
    .fetch_one(pool)
    .await?;

    let response = PostResponse {
        id: post.id,
        title: post.title,
        content: post.content,
        user_id: post.user_id,
        created_at: post.created_at.unwrap_or_else(chrono::Utc::now),
        updated_at: post.updated_at.unwrap_or_else(chrono::Utc::now),
    };

    Ok(Json(response))
}

pub async fn delete(
    Extension(auth_user): Extension<AuthUser>,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> AppResult<()> {
    let pool = &state.pool;

    let result = sqlx::query!(
        "DELETE FROM posts WHERE id = $1 AND user_id = $2",
        id,
        auth_user.id
    )
    .execute(pool)
    .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("Post not found".to_string()));
    }

    Ok(())
}
