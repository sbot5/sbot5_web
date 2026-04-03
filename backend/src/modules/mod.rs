//! 路由模块聚合
//!
//! # 完整路由表
//!
//! ## 文章 (CRUD + SEO)
//! | 方法   | 路径                 | 鉴权 | 说明                  |
//! |--------|---------------------|------|-----------------------|
//! | GET    | /posts              | 否   | 文章列表（分页+搜索） |
//! | GET    | /posts/:id          | 否   | 按 UUID 获取文章      |
//! | GET    | /post/:slug         | 否   | 按 slug 获取文章(SEO) |
//! | POST   | /admin/posts        | JWT  | 新建文章              |
//! | PATCH  | /admin/posts/:id    | JWT  | 更新文章              |
//! | DELETE | /admin/posts/:id    | JWT  | 删除文章              |
//!
//! ## 评论
//! | GET    | /posts/:id/comments          | 否   | 文章评论列表(已审核) |
//! | POST   | /posts/:id/comments          | 否   | 提交评论(待审核)     |
//! | GET    | /admin/comments              | JWT  | 管理员评论列表       |
//! | PATCH  | /admin/comments/:id          | JWT  | 审核评论             |
//! | DELETE | /admin/comments/:id          | JWT  | 删除评论             |
//!
//! ## 认证
//! | POST   | /auth/register      | 否   | 注册(需邀请码)        |
//! | POST   | /auth/login         | 否   | 登录                  |
//!
//! ## 文件上传
//! | POST   | /api/upload         | JWT  | 上传图片              |
//!
//! ## AI 对话
//! | POST   | /api/chat           | 否   | Gemini 对话           |
//!
//! ## 设置
//! | GET    | /api/settings       | 否   | 获取博客设置          |
//! | PATCH  | /api/settings       | JWT  | 更新博客设置          |
//!
//! ## SEO
//! | GET    | /feed.xml           | 否   | RSS Feed              |
//! | GET    | /sitemap.xml        | 否   | Sitemap               |

use axum::{routing::{get, patch, post}, Router};

use crate::AppState;

pub mod auth;
pub mod chat;
pub mod comment;
pub mod feed;
pub mod post;
pub mod settings;
pub mod sitemap;
pub mod upload;

/// 创建完整路由树
pub fn create_router(state: AppState) -> Router {
    Router::new()
        // ── 文章 ─────────────────────────────────────────────────
        .route("/posts",          get(post::list_posts))
        .route("/posts/{id}",     get(post::get_post))
        .route("/post/{slug}",    get(post::get_post_by_slug))
        .route("/admin/posts",    post(post::create_post))
        .route("/admin/posts/{id}",
            patch(post::update_post).delete(post::delete_post),
        )

        // ── 评论 ─────────────────────────────────────────────────
        .route("/posts/{post_id}/comments",
            get(comment::list_comments).post(comment::create_comment),
        )
        .route("/admin/comments",     get(comment::list_all_comments))
        .route("/admin/comments/{id}",
            patch(comment::moderate_comment).delete(comment::delete_comment),
        )

        // ── 认证 ─────────────────────────────────────────────────
        .route("/auth/register", post(auth::register))
        .route("/auth/login",    post(auth::login))

        // ── 文件上传 ─────────────────────────────────────────────
        .route("/api/upload", post(upload::upload_file))

        // ── AI 对话 ──────────────────────────────────────────────
        .route("/api/chat", post(chat::chat_with_gemini))

        // ── 博客设置 ─────────────────────────────────────────────
        .route("/api/settings",
            get(settings::get_settings).patch(settings::update_settings),
        )

        // ── SEO ──────────────────────────────────────────────────
        .route("/feed.xml",    get(feed::rss_feed))
        .route("/sitemap.xml", get(sitemap::sitemap))

        .with_state(state)
}
