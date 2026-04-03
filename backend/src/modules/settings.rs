//! 博客设置路由

use axum::{extract::State, Json};
use serde::{Deserialize, Serialize};

use crate::AppState;
use crate::error::AppError;
use crate::utils::jwt::AuthenticatedUser;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct BlogSettings {
    pub blog_title: String,
    pub primary_color: String,
    pub secondary_color: String,
    pub notice: String,
}

/// GET /api/settings（公开）
pub async fn get_settings(
    State(state): State<AppState>,
) -> Result<Json<BlogSettings>, AppError> {
    let settings = sqlx::query_as::<_, BlogSettings>(
        "SELECT blog_title, primary_color, secondary_color, notice
         FROM blog_settings WHERE id = 1",
    )
    .fetch_one(&state.db)
    .await?;

    Ok(Json(settings))
}

/// PATCH /api/settings（需要 JWT）
pub async fn update_settings(
    _user: AuthenticatedUser,
    State(state): State<AppState>,
    Json(payload): Json<BlogSettings>,
) -> Result<Json<BlogSettings>, AppError> {
    let settings = sqlx::query_as::<_, BlogSettings>(
        "UPDATE blog_settings
         SET blog_title=$1, primary_color=$2, secondary_color=$3, notice=$4
         WHERE id = 1
         RETURNING blog_title, primary_color, secondary_color, notice",
    )
    .bind(&payload.blog_title)
    .bind(&payload.primary_color)
    .bind(&payload.secondary_color)
    .bind(&payload.notice)
    .fetch_one(&state.db)
    .await?;

    Ok(Json(settings))
}
