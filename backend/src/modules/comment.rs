//! 评论路由模块 (P3)
//!
//! 游客可提交评论（需审核后才显示），管理员可审核/删除。
//!
//! 评论流程：
//! 1. 游客提交 → status = "pending"
//! 2. 管理员审核 → status = "approved" / "rejected"
//! 3. 只有 "approved" 的评论对外展示

use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;
use validator::Validate;

use crate::AppState;
use crate::error::AppError;
use crate::model::{Comment, CreateComment};
use crate::utils::jwt::AuthenticatedUser;

/// GET /posts/:post_id/comments
///
/// 获取某篇文章的所有已审核评论（公开接口）。
pub async fn list_comments(
    State(state): State<AppState>,
    Path(post_id): Path<Uuid>,
) -> Result<Json<Vec<Comment>>, AppError> {
    let comments = sqlx::query_as::<_, Comment>(
        "SELECT id, post_id, author_name, author_email, content, status, created_at
         FROM comments
         WHERE post_id = $1 AND status = 'approved'
         ORDER BY created_at ASC",
    )
    .bind(post_id)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(comments))
}

/// POST /posts/:post_id/comments
///
/// 游客提交评论，初始状态为 "pending"（待审核）。
pub async fn create_comment(
    State(state): State<AppState>,
    Path(post_id): Path<Uuid>,
    Json(payload): Json<CreateComment>,
) -> Result<(StatusCode, Json<Comment>), AppError> {
    payload.validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    // 确认文章存在
    let exists: bool = sqlx::query_scalar(
        "SELECT EXISTS(SELECT 1 FROM posts WHERE id = $1 AND published = true)",
    )
    .bind(post_id)
    .fetch_one(&state.db)
    .await?;

    if !exists {
        return Err(AppError::NotFound("文章不存在或未发布".into()));
    }

    let comment = sqlx::query_as::<_, Comment>(
        "INSERT INTO comments (post_id, author_name, author_email, content, status)
         VALUES ($1, $2, $3, $4, 'pending')
         RETURNING id, post_id, author_name, author_email, content, status, created_at",
    )
    .bind(post_id)
    .bind(&payload.author_name)
    .bind(&payload.author_email)
    .bind(&payload.content)
    .fetch_one(&state.db)
    .await?;

    Ok((StatusCode::CREATED, Json(comment)))
}

/// GET /admin/comments?status=pending (需要 JWT)
///
/// 管理员查看评论列表，可按状态过滤。
pub async fn list_all_comments(
    _user: AuthenticatedUser,
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<CommentFilter>,
) -> Result<Json<Vec<Comment>>, AppError> {
    let comments = sqlx::query_as::<_, Comment>(
        "SELECT id, post_id, author_name, author_email, content, status, created_at
         FROM comments
         WHERE ($1::text IS NULL OR status = $1)
         ORDER BY created_at DESC
         LIMIT 50",
    )
    .bind(&params.status)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(comments))
}

#[derive(Debug, serde::Deserialize)]
pub struct CommentFilter {
    pub status: Option<String>,
}

/// PATCH /admin/comments/:id (需要 JWT)
///
/// 审核评论：approved 或 rejected。
pub async fn moderate_comment(
    _user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<ModerateRequest>,
) -> Result<Json<Comment>, AppError> {
    // 校验状态值
    if !["approved", "rejected"].contains(&payload.status.as_str()) {
        return Err(AppError::ValidationError(
            "状态只能是 approved 或 rejected".into(),
        ));
    }

    let comment = sqlx::query_as::<_, Comment>(
        "UPDATE comments SET status = $1
         WHERE id = $2
         RETURNING id, post_id, author_name, author_email, content, status, created_at",
    )
    .bind(&payload.status)
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound("评论不存在".into()))?;

    Ok(Json(comment))
}

#[derive(Debug, serde::Deserialize)]
pub struct ModerateRequest {
    pub status: String,
}

/// DELETE /admin/comments/:id (需要 JWT)
pub async fn delete_comment(
    _user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let result = sqlx::query("DELETE FROM comments WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await?;

    if result.rows_affected() == 0 {
        return Err(AppError::NotFound("评论不存在".into()));
    }
    Ok(StatusCode::NO_CONTENT)
}
