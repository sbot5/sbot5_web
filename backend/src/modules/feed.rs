//! RSS Feed 生成模块 (P2)
//!
//! 生成标准 RSS 2.0 XML，供 RSS 阅读器订阅。
//! 路由：GET /feed.xml

use axum::{
    extract::State,
    http::{header, StatusCode},
    response::IntoResponse,
};
use rss::{ChannelBuilder, ItemBuilder};

use crate::AppState;
use crate::error::AppError;

/// GET /feed.xml
///
/// 生成包含最新 20 篇已发布文章的 RSS Feed。
pub async fn rss_feed(
    State(state): State<AppState>,
) -> Result<impl IntoResponse, AppError> {
    // 读取博客标题
    let blog_title: String = sqlx::query_scalar(
        "SELECT blog_title FROM blog_settings WHERE id = 1",
    )
    .fetch_one(&state.db)
    .await
    .unwrap_or_else(|_| "My Blog".to_string());

    // 查询最新 20 篇已发布文章
    let posts = sqlx::query_as::<_, crate::model::Post>(
        "SELECT id, title, content, published, category, tags,
                slug, summary, cover_image,
                created_at, updated_at, published_at
         FROM posts
         WHERE published = true
         ORDER BY created_at DESC
         LIMIT 20",
    )
    .fetch_all(&state.db)
    .await?;

    // 构建 RSS items
    let items: Vec<rss::Item> = posts
        .iter()
        .map(|post| {
            // 优先使用 slug 构建链接，否则使用 UUID
            let link = post
                .slug
                .as_ref()
                .map(|s| format!("/post/{}", s))
                .unwrap_or_else(|| format!("/post/{}", post.id));

            // 描述：优先使用摘要，否则截取正文前 200 字
            let description = post
                .summary
                .clone()
                .unwrap_or_else(|| {
                    let plain = post.content
                        .replace('#', "")
                        .replace('*', "")
                        .replace('`', "");
                    if plain.len() > 200 {
                        format!("{}...", &plain[..200])
                    } else {
                        plain
                    }
                });

            ItemBuilder::default()
                .title(Some(post.title.clone()))
                .link(Some(link))
                .description(Some(description))
                .pub_date(Some(post.created_at.to_rfc2822()))
                .guid(Some(rss::Guid {
                    value: post.id.to_string(),
                    permalink: false,
                }))
                .categories(
                    post.category
                        .iter()
                        .map(|c| rss::Category {
                            name: c.clone(),
                            domain: None,
                        })
                        .collect::<Vec<_>>(),
                )
                .build()
        })
        .collect();

    // 构建 RSS Channel
    let channel = ChannelBuilder::default()
        .title(blog_title)
        .description("个人博客 RSS Feed".to_string())
        .items(items)
        .build();

    let xml = channel.to_string();

    Ok((
        StatusCode::OK,
        [(header::CONTENT_TYPE, "application/rss+xml; charset=utf-8")],
        xml,
    ))
}
