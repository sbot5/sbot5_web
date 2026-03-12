use axum::{
    extract::{Path, State, Query},
    http::StatusCode,
    routing::{get, post, patch, delete},
    Json, Router,
};
use sqlx::PgPool;
use uuid::Uuid;
use validator::Validate;
use crate::model::{Post, CreatePost, UpdatePost, PostQuery};
use crate::error::AppError;

use crate::auth::{AuthenticatedUser, LoginRequest, AuthResponse, create_token};
use bcrypt::{hash, verify, DEFAULT_COST};

/// 路由聚合器
pub fn post_routes() -> Router<PgPool> {
    Router::new()
        // 公共路由
        .route("/posts", get(list_posts))
        .route("/posts/:id", get(get_post))
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        // 受保护路由 (需要 Bearer Token)
        .route("/posts", post(create_post))
        .route("/posts/:id", patch(update_post).delete(delete_post))
}

// 1. 用户注册
async fn register(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<StatusCode, AppError> {
    let password_hash = hash(payload.password, DEFAULT_COST)
        .map_err(|_| AppError::Internal("Password hashing failed".into()))?;

    sqlx::query("INSERT INTO users (username, password_hash) VALUES ($1, $2)")
        .bind(payload.username)
        .bind(password_hash)
        .execute(&pool)
        .await?;

    Ok(StatusCode::CREATED)
}

// 2. 用户登录
async fn login(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, AppError> {
    let user = sqlx::query!("SELECT id, password_hash FROM users WHERE username = $1", payload.username)
        .fetch_optional(&pool)
        .await?
        .ok_or_else(|| AppError::ValidationError("Invalid username or password".into()))?;

    if !verify(payload.password, &user.password_hash).unwrap_or(false) {
        return Err(AppError::ValidationError("Invalid username or password".into()));
    }

    let token = create_token(user.id)?;
    Ok(Json(AuthResponse { token }))
}

// 3. 创建新文章 (添加了 AuthenticatedUser 提取器)
async fn create_post(
    _user: AuthenticatedUser, // 只要提取成功，就代表已认证
    State(pool): State<PgPool>,
    Json(payload): Json<CreatePost>,
) -> Result<(StatusCode, Json<Post>), AppError> {
    payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
    // ...
}

// 4. 更新文章 (受保护)
async fn update_post(
    _user: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdatePost>,
) -> Result<Json<Post>, AppError> {
    // ...
}

// 5. 删除文章 (受保护)
async fn delete_post(
    _user: AuthenticatedUser,
    State(pool): State<PgPool>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    // ...
}

