use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub published: bool,
    pub category: Option<String>,
    pub tags: Option<String>, // 以逗号分隔的字符串
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreatePost {
    #[validate(length(min = 1, max = 255))]
    pub title: String,
    #[validate(length(min = 1))]
    pub content: String,
    pub category: Option<String>,
    pub tags: Option<String>,
    #[serde(default)]
    pub published: bool,
}

#[derive(Debug, Deserialize, Validate)]
pub struct UpdatePost {
    pub title: Option<String>,
    pub content: Option<String>,
    pub category: Option<String>,
    pub tags: Option<String>,
    pub published: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct PostQuery {
    pub search: Option<String>,
    pub category: Option<String>,
    pub published: Option<bool>,
    #[serde(default = "default_limit")]
    pub limit: i64,
    #[serde(default = "default_offset")]
    pub offset: i64,
}

fn default_limit() -> i64 { 10 }
fn default_offset() -> i64 { 0 }
