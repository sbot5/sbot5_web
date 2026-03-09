use axum::{
    extract::State,
    routing::get,
    Router,
};
use dotenvy::dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;
use std::net::SocketAddr;
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use chrono::{DateTime, Utc};

// #[derive(...)] 是 Rust 的宏，帮我们自动实现一些功能
// Serialize/Deserialize: 允许它和 JSON 互相转换（给前端用）
// FromRow: 允许 sqlx 直接把数据库里的一行数据转换成这个结构体
#[derive(Serialize, Deserialize, FromRow)]
pub struct Post {
    pub id: Uuid,
    pub title: String,
    pub content: String,
    pub published: bool,
    pub created_at: DateTime<Utc>,
}

// 1. 定义我们整个应用的“全局状态”
// #[derive(Clone)] 是必须的，因为 Axum 会为每个请求克隆一份 State 的引用
#[derive(Clone)]
struct AppState {
    pool: PgPool, // 这就是我们的数据库连接池
}

// 2. 修改 Handler：通过 State 提取器，把连接池拿出来用
async fn health_check(State(state): State<AppState>) -> &'static str {
    // 简单测试一下：尝试从连接池获取一个连接
    // 实际业务中，我们会在这一步执行 SQL 查询
    match state.pool.acquire().await {
        Ok(_) => "Hello, Blog! Database connected successfully.",
        Err(_) => "Database connection failed!",
    }
}

#[tokio::main]
async fn main() {
    // 加载 .env 文件中的环境变量
    dotenv().ok();
    
    // 获取数据库连接字符串
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // 3. 建立数据库连接池
    println!("Connecting to database...");
    let pool = PgPoolOptions::new()
        .max_connections(5) // 最大连接数设为 5（个人博客足够了）
        .connect(&db_url)
        .await
        .expect("Failed to create database connection pool");

    // 4. 初始化应用状态
    let app_state = AppState { pool };

    // 5. 构建路由，并通过 .with_state() 把状态注入进去
    let app = Router::new()
        .route("/", get(health_check))
        .with_state(app_state);

    // 启动服务器
    let addr = SocketAddr::from(([0, 0, 0, 0], 8000));
    println!("Backend server is running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}