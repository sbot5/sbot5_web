//! 数据模型模块
//!
//! 定义所有数据库实体和请求/响应结构体。

use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;

// ═══════════════════════════════════════════════════════════
// 文章
// ═══════════════════════════════════════════════════════════

/// 博客文章（对应 `posts` 表）
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    /// Markdown 正文
    pub content: String,
    pub published: bool,
    pub category: Option<String>,
    /// 标签数组（PostgreSQL TEXT[]）
    pub tags: Vec<String>,
    /// SEO 友好 URL 路径，如 "my-first-post"
    pub slug: Option<String>,
    /// 独立摘要，列表页展示用
    pub summary: Option<String>,
    /// 封面图 URL
    pub cover_image: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    /// 定时发布时间（未设置表示立即发布或草稿）
    pub published_at: Option<DateTime<Utc>>,
}

/// 创建文章请求体
#[derive(Debug, Deserialize, Validate)]
pub struct CreatePost {
    #[validate(length(min = 1, max = 255, message = "标题长度必须在 1-255 之间"))]
    pub title: String,
    #[validate(length(min = 1, message = "内容不能为空"))]
    pub content: String,
    pub category: Option<String>,
    /// 标签数组
    #[serde(default)]
    pub tags: Vec<String>,
    pub summary: Option<String>,
    pub cover_image: Option<String>,
    /// 自定义 slug（留空则自动生成）
    pub slug: Option<String>,
    #[serde(default)]
    pub published: bool,
    /// 定时发布时间
    pub published_at: Option<DateTime<Utc>>,
}

/// 更新文章请求体（所有字段可选）
#[derive(Debug, Deserialize, Validate)]
pub struct UpdatePost {
    #[validate(length(min = 1, max = 255))]
    pub title: Option<String>,
    pub content: Option<String>,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
    pub summary: Option<String>,
    pub cover_image: Option<String>,
    pub slug: Option<String>,
    pub published: Option<bool>,
    pub published_at: Option<DateTime<Utc>>,
}

/// 文章列表查询参数
#[derive(Debug, Deserialize)]
pub struct PostQuery {
    /// 全文搜索关键词
    pub search: Option<String>,
    /// 按分类过滤
    pub category: Option<String>,
    /// 按标签过滤（精确匹配单个标签）
    pub tag: Option<String>,
    /// 按发布状态过滤
    pub published: Option<bool>,
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default)]
    pub offset: i64,
}

fn default_limit() -> i64 { 10 }

/// 带分页信息的文章列表响应
#[derive(Debug, Serialize)]
pub struct PaginatedPosts {
    /// 文章列表
    pub items: Vec<Post>,
    /// 符合条件的总数（用于前端渲染分页）
    pub total: i64,
    /// 当前偏移量
    pub offset: i64,
    /// 每页数量
    pub limit: i64,
}

// ═══════════════════════════════════════════════════════════
// 评论 (P3)
// ═══════════════════════════════════════════════════════════

/// 评论（对应 `comments` 表）
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Comment {
    pub id: Uuid,
    pub post_id: Uuid,
    pub author_name: String,
    /// 邮箱不在 API 响应中暴露（标记 skip_serializing）
    #[serde(skip_serializing)]
    pub author_email: Option<String>,
    pub content: String,
    /// 审核状态：pending / approved / rejected
    pub status: String,
    pub created_at: DateTime<Utc>,
}

/// 创建评论请求体
#[derive(Debug, Deserialize, Validate)]
pub struct CreateComment {
    #[validate(length(min = 1, max = 100, message = "昵称长度 1-100"))]
    pub author_name: String,
    #[validate(email(message = "邮箱格式不正确"))]
    pub author_email: Option<String>,
    #[validate(length(min = 1, max = 2000, message = "评论内容 1-2000 字"))]
    pub content: String,
}

// ═══════════════════════════════════════════════════════════
// 文件上传 (P1)
// ═══════════════════════════════════════════════════════════

/// 上传文件记录
#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Upload {
    pub id: Uuid,
    pub original_name: String,
    pub file_path: String,
    pub mime_type: String,
    pub file_size: i64,
    pub created_at: DateTime<Utc>,
}

/// 上传成功响应
#[derive(Debug, Serialize)]
pub struct UploadResponse {
    /// 文件访问 URL（如 /uploads/2026/04/abc123.png）
    pub url: String,
    pub id: Uuid,
    pub original_name: String,
    pub file_size: i64,
}
