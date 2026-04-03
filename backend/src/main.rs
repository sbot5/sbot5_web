//! 博客后端入口文件
//!
//! 启动流程：
//! 1. 初始化结构化日志
//! 2. 加载 .env 环境变量
//! 3. 建立 PostgreSQL 连接池
//! 4. 自动执行数据库迁移
//! 5. 配置 CORS（根据环境变量设置白名单）
//! 6. 配置速率限制（防暴力破解）
//! 7. 创建 uploads 目录
//! 8. 挂载路由，启动 HTTP 服务
//!
//! # 必需环境变量
//! | 变量名          | 说明                              |
//! |----------------|-----------------------------------|
//! | DATABASE_URL   | PostgreSQL 连接字符串              |
//! | JWT_SECRET     | JWT 签名密钥（≥32 字节随机字符串） |
//!
//! # 可选环境变量
//! | 变量名          | 默认值                            |
//! |----------------|-----------------------------------|
//! | HOST           | 127.0.0.1                         |
//! | PORT           | 3000                              |
//! | CORS_ORIGIN    | http://localhost:5173（多个用逗号） |
//! | REGISTER_CODE  | 未设置则关闭注册                   |
//! | GEMINI_API_KEY | AI 对话功能密钥                    |
//! | RUST_LOG       | backend=debug,tower_http=debug     |

use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::cors::CorsLayer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod error;
mod model;
mod modules;
mod utils;

/// 应用级共享状态
///
/// 通过 Axum State 注入到所有路由，避免每次请求重复读取环境变量
/// 或创建新的 HTTP 客户端。
#[derive(Clone)]
pub struct AppState {
    /// PostgreSQL 连接池
    pub db: sqlx::PgPool,
    /// 预创建的 reqwest 客户端（复用 TCP 连接）
    pub http_client: reqwest::Client,
    /// JWT 签名密钥
    pub jwt_secret: Arc<String>,
    /// 注册邀请码（None 表示关闭注册）
    pub register_code: Option<String>,
    /// 文件上传目录的绝对路径
    pub upload_dir: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ── 1. 初始化日志 ──────────────────────────────────────────
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            env::var("RUST_LOG")
                .unwrap_or_else(|_| "backend=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // ── 2. 加载 .env 文件 ──────────────────────────────────────
    dotenv().ok();

    // ── 3. 建立数据库连接池 ─────────────────────────────────────
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL 环境变量未设置");

    tracing::info!("连接数据库...");
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("数据库连接失败");

    // ── 4. 执行迁移 ─────────────────────────────────────────────
    tracing::info!("执行数据库迁移...");
    sqlx::migrate!().run(&pool).await.expect("数据库迁移失败");

    // ── 5. CORS 配置 ────────────────────────────────────────────
    // 从 CORS_ORIGIN 读取允许的源，多个用逗号分隔
    // 未设置时默认允许 localhost:5173（Vite 开发服务器）
    let cors = build_cors_layer();

    // ── 6. 构建共享状态 ─────────────────────────────────────────
    let jwt_secret = env::var("JWT_SECRET")
        .expect("JWT_SECRET 环境变量未设置！");

    let register_code = env::var("REGISTER_CODE").ok();
    if register_code.is_none() {
        tracing::warn!("REGISTER_CODE 未设置，注册功能已关闭");
    }

    // 创建 uploads 目录
    let upload_dir = env::var("UPLOAD_DIR")
        .unwrap_or_else(|_| "./uploads".into());
    tokio::fs::create_dir_all(&upload_dir).await?;

    let state = AppState {
        db: pool,
        http_client: reqwest::Client::new(),
        jwt_secret: Arc::new(jwt_secret),
        register_code,
        upload_dir: upload_dir.clone(),
    };

    // ── 7. 组装路由并启动服务器 ─────────────────────────────────
    let app = modules::create_router(state)
        .layer(cors)
        // 静态文件服务：映射 /uploads/* 到磁盘
        .nest_service(
            "/uploads",
            tower_http::services::ServeDir::new(&upload_dir),
        );

    let host = env::var("HOST").unwrap_or_else(|_| "127.0.0.1".into());
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "3000".into())
        .parse()
        .expect("PORT 必须是有效端口号");

    let addr: SocketAddr = format!("{}:{}", host, port).parse()?;
    tracing::info!("博客后端启动: http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

/// 根据环境变量构建 CORS 层
///
/// - `CORS_ORIGIN` 设置时：仅允许指定的源（逗号分隔）
/// - 未设置时：默认允许 `http://localhost:5173`
fn build_cors_layer() -> CorsLayer {
    use axum::http::{HeaderValue, Method};
    use tower_http::cors::AllowOrigin;

    let origins_str = env::var("CORS_ORIGIN")
        .unwrap_or_else(|_| "http://localhost:5173".into());

    let origins: Vec<HeaderValue> = origins_str
        .split(',')
        .filter_map(|s| s.trim().parse().ok())
        .collect();

    CorsLayer::new()
        .allow_origin(AllowOrigin::list(origins))
        .allow_methods([
            Method::GET,
            Method::POST,
            Method::PATCH,
            Method::DELETE,
            Method::OPTIONS,
        ])
        .allow_headers(tower_http::cors::Any)
}
