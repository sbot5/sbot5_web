use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::env;
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod model;
mod router;
mod error;
mod auth;

use tower_http::cors::{Any, CorsLayer};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            env::var("RUST_LOG").unwrap_or_else(|_| "backend=debug,tower_http=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in .env file");
    
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await?;

    // --- 修复阶段：强制重置迁移记录 ---
    // 只有在遇到 VersionMismatch 时才需要执行。执行一次后即可删掉。
    tracing::info!("Cleaning up migration metadata to fix VersionMismatch...");
    sqlx::query("DROP TABLE IF EXISTS _sqlx_migrations").execute(&pool).await?;

    // 运行数据库迁移 (现在会重新记录所有迁移，且不会报错)
    sqlx::migrate!().run(&pool).await?;
    tracing::info!("Database migrations applied successfully.");

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods([
            axum::http::Method::GET, 
            axum::http::Method::POST, 
            axum::http::Method::PATCH, 
            axum::http::Method::DELETE,
            axum::http::Method::OPTIONS
        ])
        .allow_headers(Any);

    let app = router::post_routes()
        .layer(cors)
        .with_state(pool);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
