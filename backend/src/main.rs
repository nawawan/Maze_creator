use axum::{Router, routing::get, routing::post};


#[tokio::main]
async fn main() {
    let app = Router::new()
            .route("/", get(|| async { "Hello, World!" }))
            .route("/health", get(|| async { "OK" }));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.expect("error: failed to bind to address");
    axum::serve(listener, app).await.expect("error: failed to serve");
}

fn create_blog_router() -> Router {
    Router::new()
        .route("/blogs", post(|| async { "Blog posts" }))
        .route("/blogs", get(|| async { "Blog list" }))
        .route("/blogs/:id", get(|| async { "Blog get by ID" }))
}