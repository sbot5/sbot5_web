use axum::{
    extract::{Path, State, Query},
    http::StatusCode,
    routing::{get, post, patch, delete},
    Json, Router,
};
use sqlx::{PgPool, Row};
use uuid::Uuid;
use validator::Validate;
use crate::model::{Post, CreatePost, UpdatePost, PostQuery};
use crate::error::AppError;

use crate::auth::{AuthenticatedUser, LoginRequest, AuthResponse, create_token};
use bcrypt::{hash, verify, DEFAULT_COST};

pub fn post_routes() -> Router<PgPool> {
    Router::new()
        .route("/posts", get(list_posts))
        .route("/posts/:id", get(get_post))
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/admin/posts", post(create_post))
        .route("/admin/posts/:id", patch(update_post).delete(delete_post))
}

async fn register(State(pool): State<PgPool>, Json(payload): Json<LoginRequest>) -> Result<StatusCode, AppError> {
    let password_hash = hash(payload.password, DEFAULT_COST).map_err(|_| AppError::Internal("Hashing failed".into()))?;
    sqlx::query("INSERT INTO users (username, password_hash) VALUES ($1, $2)").bind(payload.username).bind(password_hash).execute(&pool).await?;
    Ok(StatusCode::CREATED)
}

async fn login(State(pool): State<PgPool>, Json(payload): Json<LoginRequest>) -> Result<Json<AuthResponse>, AppError> {
    let user = sqlx::query("SELECT id, password_hash FROM users WHERE username = $1").bind(&payload.username).fetch_optional(&pool).await?.ok_or_else(|| AppError::ValidationError("User not found".into()))?;
    let id: Uuid = user.get("id");
    let password_hash: String = user.get("password_hash");
    if !verify(payload.password, &password_hash).unwrap_or(false) { return Err(AppError::ValidationError("Wrong password".into())); }
    let token = create_token(id)?;
    Ok(Json(AuthResponse { token }))
}

async fn list_posts(State(pool): State<PgPool>, Query(query): Query<PostQuery>) -> Result<Json<Vec<Post>>, AppError> {
    let sql = "SELECT id, title, content, published, category, tags, created_at FROM posts WHERE (title ILIKE $1 OR content ILIKE $1) AND ($2 IS NULL OR category = $2) AND ($3 IS NULL OR published = $3) ORDER BY created_at DESC LIMIT $4 OFFSET $5";
    let search = format!("%{}%", query.search.unwrap_or_default());
    let posts = sqlx::query_as::<_, Post>(sql).bind(search).bind(query.category).bind(query.published).bind(query.limit).bind(query.offset).fetch_all(&pool).await?;
    Ok(Json(posts))
}

async fn get_post(State(pool): State<PgPool>, Path(id): Path<Uuid>) -> Result<Json<Post>, AppError> {
    let post = sqlx::query_as::<_, Post>("SELECT id, title, content, published, category, tags, created_at FROM posts WHERE id = $1").bind(id).fetch_optional(&pool).await?.ok_or_else(|| AppError::NotFound("Not found".into()))?;
    Ok(Json(post))
}

async fn create_post(_user: AuthenticatedUser, State(pool): State<PgPool>, Json(payload): Json<CreatePost>) -> Result<(StatusCode, Json<Post>), AppError> {
    payload.validate().map_err(|e| AppError::ValidationError(e.to_string()))?;
    let post = sqlx::query_as::<_, Post>("INSERT INTO posts (title, content, published, category, tags, author_id) VALUES ($1, $2, $3, $4, $5, $6) RETURNING id, title, content, published, category, tags, created_at").bind(payload.title).bind(payload.content).bind(payload.published).bind(payload.category).bind(payload.tags).bind(_user.0).fetch_one(&pool).await?;
    Ok((StatusCode::CREATED, Json(post)))
}

async fn update_post(_user: AuthenticatedUser, State(pool): State<PgPool>, Path(id): Path<Uuid>, Json(payload): Json<UpdatePost>) -> Result<Json<Post>, AppError> {
    let post = sqlx::query_as::<_, Post>("UPDATE posts SET title = COALESCE($1, title), content = COALESCE($2, content), published = COALESCE($3, published), category = COALESCE($4, category), tags = COALESCE($5, tags) WHERE id = $6 RETURNING id, title, content, published, category, tags, created_at").bind(payload.title).bind(payload.content).bind(payload.published).bind(payload.category).bind(payload.tags).bind(id).fetch_one(&pool).await?;
    Ok(Json(post))
}

async fn delete_post(_user: AuthenticatedUser, State(pool): State<PgPool>, Path(id): Path<Uuid>) -> Result<StatusCode, AppError> {
    sqlx::query("DELETE FROM posts WHERE id = $1").bind(id).execute(&pool).await?;
    Ok(StatusCode::NO_CONTENT)
}
