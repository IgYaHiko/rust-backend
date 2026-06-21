mod  db;
mod models;
use axum::{
    routing::get,
    Router,
};
use db:: create_pool;
#[tokio::main]
async fn main() {
    // Build our application with a route
    let app = Router::new()
        .route("/", get(hello_world))
        .route("/hello", get(hello));

    // Run it with hyper on localhost:3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Server running on http://localhost:3000");
    axum::serve(listener, app).await.unwrap();
}

// Handler for GET /
async fn hello_world() -> &'static str {
    "Hello, World!"
}

// Handler for GET /hello
async fn hello() -> &'static str {
    "Hello from Axum!"
}