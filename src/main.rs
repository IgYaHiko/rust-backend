mod db;
mod models;
mod handlers;
mod auth;
mod error;
mod state;

use axum::{
    routing::{get, post, put, delete},
    Router,
    http::Method,
    middleware,
};
use tower_http::cors::{CorsLayer, Any};
use dotenv::dotenv;
use std::sync::Arc;
use tracing::info;
use tower_http::trace::TraceLayer;

use db::create_pool;
use handlers::posts;
use auth::jwt::JwtService;
use auth::middleware::auth_middleware;
use state::AppState;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Load environment variables
    dotenv().ok();
    
    // Setup logging - SIMPLIFIED
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    // Read port from env
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()?;

    // Create database connection pool
    let database_url = std::env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set");
    let pool = create_pool(&database_url).await?;
    
    info!("Database connected successfully");

    // Create JWT service
    let jwt_secret = std::env::var("JWT_SECRET")
        .expect("JWT_SECRET must be set");
    let jwt_service = Arc::new(JwtService::new(&jwt_secret));

    // Create shared state
    let state = AppState {
        pool,
        jwt_service: jwt_service.clone(),
    };

    // Build our application
    let app = Router::new()
        // Auth routes (public - no middleware)
        .route("/api/auth/register", post(auth::handlers::register))
        .route("/api/auth/login", post(auth::handlers::login))
        // Protected post routes - apply middleware to these
        .nest(
            "/api/posts",
            Router::new()
                .route("/", get(posts::get_all))
                .route("/", post(posts::create))
                .route("/:id", get(posts::get_one))
                .route("/:id", put(posts::update))
                .route("/:id", delete(posts::delete))
                .layer(middleware::from_fn_with_state(
                    jwt_service.clone(),
                    auth_middleware
                ))
        )
        // Add state
        .with_state(state)
        // Add CORS
        .layer(
            CorsLayer::new()
                .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
                .allow_origin(Any)
                .allow_headers(Any)
        )
        // Add tracing
        .layer(TraceLayer::new_for_http());

    // Start server
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port)).await?;
    info!("Server running on http://localhost:{}", port);
    
    axum::serve(listener, app).await?;
    
    Ok(())
}
