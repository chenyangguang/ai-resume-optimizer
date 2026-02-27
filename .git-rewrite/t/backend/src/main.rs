use axum::{
    routing::{get, post},
    Router,
};
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod routes;
mod services;
mod models;
mod utils;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();

    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ai_resume_optimizer=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("🚀 AI Resume Optimizer starting...");

    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    
    let db_pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await?;

    tracing::info!("✅ Database pool created");

    let redis_url = std::env::var("REDIS_URL")
        .expect("REDIS_URL must be set");
    
    let redis_client = redis::Client::open(redis_url.as_str())?;

    tracing::info!("✅ Redis client created");

    let app = Router::new()
        .route("/api/health", get(routes::health))
        // Resume routes
        .route("/api/resume/optimize", post(routes::resume::optimize))
        .route("/api/resume/score", post(routes::resume::score))
        .route("/api/resume/cover-letter", post(routes::resume::generate_cover_letter))
        .route("/api/resume/keywords", post(routes::resume::extract_keywords))
        .layer(
            CorsLayer::new()
                .allow_origin(Any)
                .allow_methods(Any)
                .allow_headers(Any),
        )
        .with_state(utils::AppState::new(db_pool, redis_client));

    let host = std::env::var("SERVER_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = std::env::var("SERVER_PORT")
        .unwrap_or_else(|_| "3002".to_string())
        .parse::<u16>()?;
    
    let addr = format!("{}:{}", host, port);
    
    tracing::info!("🌐 Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
