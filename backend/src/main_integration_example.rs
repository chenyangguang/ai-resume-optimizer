// 在 main.rs 中需要添加的内容

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use std::net::SocketAddr;

mod models;
mod routes;
mod services;
mod utils;
mod middleware;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    
    // 数据库连接
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to connect to database");

    // Redis 连接
    let redis_url = std::env::var("REDIS_URL")
        .unwrap_or_else(|_| "redis://127.0.0.1:6379".to_string());
    
    // 构建路由
    let app = Router::new()
        // 健康检查
        .route("/api/health", get(|| async { 
            serde_json::json!({
                "service": "ai-resume-optimizer",
                "status": "ok",
                "version": "0.1.0"
            })
        }))
        
        // 认证路由（新增）
        .route("/api/auth/register", post(routes::auth::register))
        .route("/api/auth/login", post(routes::auth::login))
        
        // 使用统计路由（新增）
        .route("/api/usage/check", get(routes::usage::check_usage))
        .route("/api/usage/stats", get(routes::usage::get_usage_stats))
        
        // 简历相关路由
        .route("/api/resume/optimize", post(routes::resume::optimize_resume))
        .route("/api/resume/score", post(routes::resume::score_resume))
        .route("/api/resume/cover-letter", post(routes::resume::generate_cover_letter))
        .route("/api/resume/health", get(|| async { 
            serde_json::json!({
                "service": "ai-resume-optimizer",
                "status": "ok"
            })
        }))
        
        .with_state(pool);

    // 启动服务器
    let addr = SocketAddr::from(([0, 0, 0, 0], 3002));
    println!("🚀 Server running on http://{}", addr);
    
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}
