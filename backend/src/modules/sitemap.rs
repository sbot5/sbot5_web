//! Sitemap 生成模块 (P2)
//!
//! 生成标准 sitemap.xml 供搜索引擎抓取。
//! 路由：GET /sitemap.xml

use axum::{
    extract::State,
    http::{header, StatusCode},
    response::IntoResponse,
};
use chrono::{DateTime, Utc};

use crate::AppState;
use crate::error::AppError;

/// 文章摘要（仅用于 sitemap 生成，避免加载全部字段）
#[derive(sqlx::FromRow)]
struct PostSummary {
    slug: Option<String>,
    id: uuid::Uuid,
    updated_at: DateTime<Utc>,
}

/// GET /sitemap.xml
///
/// 包含所有已发布文章的 URL 和最后更新时间。
pub async fn sitemap(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    let posts = sqlx::query_as::<_, PostSummary>(
        "SELECT id, slug, updated_at
         FROM posts
         WHERE published = true
         ORDER BY updated_at DESC",
    )
    .fetch_all(&state.db)
    .await?;

    // 手工拼接 XML（避免引入重量级 XML 库）
    let mut xml = String::from(
        r#"<?xml version="1.0" encoding="UTF-8"?>
<urlset xmlns="http://www.sitemaps.org/schemas/sitemap/0.9">
  <url>
    <loc>/</loc>
    <changefreq>daily</changefreq>
    <priority>1.0</priority>
  </url>
"#,
    );

    for post in &posts {
        let loc = post
            .slug
            .as_ref()
            .map(|s| format!("/post/{}", s))
            .unwrap_or_else(|| format!("/post/{}", post.id));

        let lastmod = post.updated_at.format("%Y-%m-%d").to_string();

        xml.push_str(&format!(
            "  <url>\n    <loc>{}</loc>\n    <lastmod>{}</lastmod>\n    <changefreq>weekly</changefreq>\n    <priority>0.8</priority>\n  </url>\n",
            loc, lastmod
        ));
    }

    xml.push_str("</urlset>\n");

    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/xml; charset=utf-8")],
        xml,
    ))
}
