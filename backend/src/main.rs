use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// 引入模块
mod model;
mod router;
mod error;
mod auth;

use tower_http::cors::{Any, CorsLayer};
use ax_http::Method; // 或者是直接使用 http::Method

#[tokio::main]
async fn main() {
    // ... 前面的代码不变 ...
    
    // 3. 构建应用
    let cors = CorsLayer::new()
        .allow_origin(Any) // 生产环境应改为具体的前端 URL
        .allow_methods([axum::http::Method::GET, axum::http::Method::POST, axum::http::Method::PATCH, axum::http::Method::DELETE])
        .allow_headers(Any);

    let app = router::post_routes()
        .layer(cors) // 应用跨域中间件
        .with_state(pool);
    
    // ... 后面的启动代码不变 ...
}
