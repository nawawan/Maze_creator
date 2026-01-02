mod handler;

use async_shutdown::ShutdownManager;
use dotenv::dotenv;
use axum::{Router, routing::get, http::StatusCode, Json};
use serde_json;
use tokio_postgres::{Client, Connection, Socket, Config};
use tokio_postgres::config::SslMode;
use openssl::ssl::{SslConnector, SslMethod};
use postgres_openssl::MakeTlsConnector;
use std::env;
use tracing_subscriber;
use tracing::info;

use crate::handler::handle_blogs::*;

#[tokio::main]
async fn main() {

    tracing_subscriber::fmt().json().init();

    dotenv().ok();

    let (client, connection) = initialize_db().await;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            info!("connection error: {}", e);
        }
    });

    let app = Router::new()
            .route("/", get(|| async { "Hello, World!" }))
            .route("/health", get(health_ok))
            .nest("/api", create_blog_router())
            .fallback(fallback);

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8000").await.expect("error: failed to bind to address");

    let shutdown = ShutdownManager::new();

    match axum::serve(listener, app).await {
        Ok(()) => {
            shutdown.trigger_shutdown(0).ok();
        },
        Err(e) => {
            info!("server error: {}", e);
            shutdown.trigger_shutdown(1).ok();
        }
    };

    let exit_code = shutdown.wait_shutdown_complete().await;
    std::process::exit(exit_code);
}

fn create_blog_router() -> Router {
    Router::new()
        .route("/blogs", get(get_blogs).post(|| async { "Blog posts" }))
        .route("/blogs/{id}", get(|| async { "Blog get by ID" }))
        .fallback(api_fallback)
}

async fn initialize_db() -> (Client, Connection<Socket, postgres_openssl::TlsStream<Socket>>) {

    let mut config = Config::new();

    let db_host = env::var("DB_HOST").expect("DB_HOST must be set");
    let db_port = env::var("DB_PORT").expect("DB_PORT must be set");
    let db_user = env::var("DB_USER").expect("DB_USER must be set");
    let db_password = env::var("DB_PASSWORD").expect("DB_PASSWORD must be set");
    let db_name = env::var("DB_NAME").expect("DB_NAME must be set");

    config.host(&db_host);
    config.port(db_port.parse::<u16>().expect("DB_PORT must be a valid u16"));
    config.user(&db_user);
    config.password(&db_password);
    config.dbname(&db_name);
    config.ssl_mode(SslMode::Require);

    let connector = SslConnector::builder(SslMethod::tls()).expect("Failed to create TLS connector");
    let tls = MakeTlsConnector::new(connector.build());

    config.connect(tls).await.expect("Failed to connect to database")
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