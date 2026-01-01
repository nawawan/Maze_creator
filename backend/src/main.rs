use axum::{Router, routing::get, routing::post, http::StatusCode, Json};
use serde_json;

#[tokio::main]
async fn main() {
    let app = Router::new()
            .route("/", get(|| async { "Hello, World!" }))
            .route("/health", get(health_ok))
            .nest("/api", create_blog_router())
            .fallback(fallback);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.expect("error: failed to bind to address");
    axum::serve(listener, app).await.expect("error: failed to serve");
}

fn create_blog_router() -> Router {
    Router::new()
        .route("/blogs", post(|| async { "Blog posts" }))
        .route("/blogs", get(|| async { "Blog list" }))
        .route("/blogs/{id}", get(|| async { "Blog get by ID" }))
        .fallback(api_fallback)
}

async fn fallback() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not Found")
}

async fn health_ok() -> (StatusCode, &'static str) {
    (StatusCode::OK, "OK")
}

async fn api_fallback() -> (StatusCode, Json<serde_json::Value>) {
    (StatusCode::NOT_FOUND, Json(serde_json::json!({
        "status": "Not Found"
    })))
}