//! 博客文章路由处理模块
//!
//! CRUD + 分页 + slug 路由 + 标签数组查询

use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use uuid::Uuid;
use validator::Validate;

use crate::AppState;
use crate::error::AppError;
use crate::model::{CreatePost, PaginatedPosts, Post, PostQuery, UpdatePost};
use crate::utils::jwt::AuthenticatedUser;

/// GET /posts
///
/// 返回带分页信息的文章列表，支持搜索、分类、标签过滤。
pub async fn list_posts(
    State(state): State<AppState>,
    Query(query): Query<PostQuery>,
) -> Result<Json<PaginatedPosts>, AppError> {
    let search_pattern = format!("%{}%", query.search.unwrap_or_default());

    // 查询符合条件的总数（用于前端分页 UI）
    let total: i64 = sqlx::query_scalar(
        "SELECT COUNT(*)
         FROM posts
         WHERE (title ILIKE $1 OR content ILIKE $1)
           AND ($2::text IS NULL OR category = $2)
           AND ($3::text IS NULL OR $3 = ANY(tags))
           AND ($4::bool IS NULL OR published = $4)",
    )
    .bind(&search_pattern)
    .bind(&query.category)
    .bind(&query.tag)
    .bind(query.published)
    .fetch_one(&state.db)
    .await?;

    // 查询当前页数据
    let items = sqlx::query_as::<_, Post>(
        "SELECT id, title, content, published, category, tags,
                slug, summary, cover_image,
                created_at, updated_at, published_at
         FROM posts
         WHERE (title ILIKE $1 OR content ILIKE $1)
           AND ($2::text IS NULL OR category = $2)
           AND ($3::text IS NULL OR $3 = ANY(tags))
           AND ($4::bool IS NULL OR published = $4)
         ORDER BY created_at DESC
         LIMIT $5 OFFSET $6",
    )
    .bind(&search_pattern)
    .bind(&query.category)
    .bind(&query.tag)
    .bind(query.published)
    .bind(query.limit)
    .bind(query.offset)
    .fetch_all(&state.db)
    .await?;

    Ok(Json(PaginatedPosts {
        items,
        total,
        offset: query.offset,
        limit: query.limit,
    }))
}

/// GET /posts/:id
///
/// 按 UUID 获取文章详情。
pub async fn get_post(
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<Post>, AppError> {
    let post = sqlx::query_as::<_, Post>(
        "SELECT id, title, content, published, category, tags,
                slug, summary, cover_image,
                created_at, updated_at, published_at
         FROM posts WHERE id = $1",
    )
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("文章 {} 不存在", id)))?;

    Ok(Json(post))
}

/// GET /post/:slug  (P2: SEO 友好路由)
///
/// 按 slug 获取文章，用于友好 URL。
pub async fn get_post_by_slug(
    State(state): State<AppState>,
    Path(slug_str): Path<String>,
) -> Result<Json<Post>, AppError> {
    let post = sqlx::query_as::<_, Post>(
        "SELECT id, title, content, published, category, tags,
                slug, summary, cover_image,
                created_at, updated_at, published_at
         FROM posts WHERE slug = $1",
    )
    .bind(&slug_str)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("文章 '{}' 不存在", slug_str)))?;

    Ok(Json(post))
}

/// POST /admin/posts (需要 JWT)
///
/// 创建新文章。自动生成 slug（如未提供）。
pub async fn create_post(
    user: AuthenticatedUser,
    State(state): State<AppState>,
    Json(payload): Json<CreatePost>,
) -> Result<(StatusCode, Json<Post>), AppError> {
    payload
        .validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    // slug: 优先使用自定义值，否则从标题自动生成
    let post_slug = payload
        .slug
        .filter(|s| !s.is_empty())
        .unwrap_or_else(|| slug::slugify(&payload.title));

    // 发布时间：已发布则设为当前时间
    let pub_at = if payload.published {
        Some(payload.published_at.unwrap_or_else(chrono::Utc::now))
    } else {
        payload.published_at
    };

    let post = sqlx::query_as::<_, Post>(
        "INSERT INTO posts (title, content, published, category, tags,
                            slug, summary, cover_image, author_id, published_at)
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
         RETURNING id, title, content, published, category, tags,
                   slug, summary, cover_image, created_at, updated_at, published_at",
    )
    .bind(&payload.title)
    .bind(&payload.content)
    .bind(payload.published)
    .bind(&payload.category)
    .bind(&payload.tags)
    .bind(&post_slug)
    .bind(&payload.summary)
    .bind(&payload.cover_image)
    .bind(user.0)
    .bind(pub_at)
    .fetch_one(&state.db)
    .await?;

    Ok((StatusCode::CREATED, Json(post)))
}

/// PATCH /admin/posts/:id (需要 JWT)
///
/// 部分更新文章。使用 COALESCE 仅更新提供的字段。
pub async fn update_post(
    _user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
    Json(payload): Json<UpdatePost>,
) -> Result<Json<Post>, AppError> {
    payload
        .validate()
        .map_err(|e| AppError::ValidationError(e.to_string()))?;

    let post = sqlx::query_as::<_, Post>(
        "UPDATE posts SET
           title        = COALESCE($1, title),
           content      = COALESCE($2, content),
           published    = COALESCE($3, published),
           category     = COALESCE($4, category),
           tags         = COALESCE($5, tags),
           slug         = COALESCE($6, slug),
           summary      = COALESCE($7, summary),
           cover_image  = COALESCE($8, cover_image),
           published_at = COALESCE($9, published_at),
           updated_at   = NOW()
         WHERE id = $10
         RETURNING id, title, content, published, category, tags,
                   slug, summary, cover_image, created_at, updated_at, published_at",
    )
    .bind(payload.title)
    .bind(payload.content)
    .bind(payload.published)
    .bind(payload.category)
    .bind(payload.tags.as_deref())
    .bind(payload.slug)
    .bind(payload.summary)
    .bind(payload.cover_image)
    .bind(payload.published_at)
    .bind(id)
    .fetch_optional(&state.db)
    .await?
    .ok_or_else(|| AppError::NotFound(format!("文章 {} 不存在", id)))?;

    Ok(Json(post))
}

/// DELETE /admin/posts/:id (需要 JWT)
///
/// 删除文章，未找到时返回 404。
pub async fn delete_post(
    _user: AuthenticatedUser,
    State(state): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    let result = sqlx::query("DELETE FROM posts WHERE id = $1")
        .bind(id)
        .execute(&state.db)
        .await?;

    // 检查是否实际删除了行
    if result.rows_affected() == 0 {
        return Err(AppError::NotFound(format!("文章 {} 不存在", id)));
    }

    Ok(StatusCode::NO_CONTENT)
}
