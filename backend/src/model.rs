use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;

/// 实体类：Post
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub published: bool,
    pub created_at: DateTime<Utc>,
}

/// 请求参数：创建文章 (带校验)
#[derive(Debug, Deserialize, Validate)]
pub struct CreatePost {
    #[validate(length(min = 1, max = 255, message = "标题长度必须在 1-255 之间"))]
    pub title: String,
    #[validate(length(min = 1, message = "内容不能为空"))]
    pub content: String,
    #[serde(default)]
    pub published: bool,
}

/// 请求参数：更新文章 (带校验)
#[derive(Debug, Deserialize, Validate)]
pub struct UpdatePost {
    #[validate(length(min = 1, max = 255))]
    pub title: Option<String>,
    pub content: Option<String>,
    pub published: Option<bool>,
}

/// 查询参数：博客列表查询 (支持搜索与分页)
#[derive(Debug, Deserialize)]
pub struct PostQuery {
    pub search: Option<String>,    // 搜索关键词
    pub published: Option<bool>,   // 是否只查已发布
    #[serde(default = "default_limit")]
    pub limit: i64,                // 每页数量
    #[serde(default = "default_offset")]
    pub offset: i64,               // 偏移量
}

fn default_limit() -> i64 { 10 }
fn default_offset() -> i64 { 0 }
