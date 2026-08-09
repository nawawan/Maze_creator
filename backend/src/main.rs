use async_shutdown::ShutdownManager;
use aws_config::{BehaviorVersion, Region};
use aws_sdk_s3::Client;
use aws_sdk_s3::config::Credentials;
use axum::{Json, Router, extract::State, http::StatusCode, routing::get, routing::post};
use dotenv::dotenv;
use serde_json;
use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::env;
use tracing::info;
use tracing_subscriber;

use std::sync::Arc;

use handler::handler::*;
use shared::config::Config;
use shared::jwks_cache::JwksCache;
use storage::repository::*;
use usecase::service::service::*;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().json().init();

    dotenv().ok();

    let pool = initialize_db().await;
    let r2_client = initialize_cloud_storage().await;
    let config = Config {
        host: env::var("PAGE_HOST").expect("PAGE_HOST must be set"),
        env: env::var("ENV").expect("ENV must be set"),
        cf_access_team_domain: env::var("CF_ACCESS_TEAM_DOMAIN")
            .expect("CF_ACCESS_TEAM_DOMAIN must be set"),
        cf_access_aud: env::var("CF_ACCESS_AUD").expect("CF_ACCESS_AUD must be set"),
        blog_r2_bucket: env::var("BLOG_R2_BUCKET").expect("BLOG_R2_BUCKET must be set"),
    };

    let jwks_cache = JwksCache::new(config.cf_access_team_domain.clone());

    let repository = Box::new(Repository::new(pool.clone(), r2_client, config.clone()));
    let service = Arc::new(Service::new(config, repository, jwks_cache));

    let app = Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .nest("/health", create_health_router(pool))
        .nest("/api", create_blog_router(service.clone()))
        .nest("/api", create_trajectory_router(service.clone()))
        .fallback(fallback);
    let port = env::var("PORT").unwrap_or_else(|_| "8080".to_string());
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .expect("error: failed to bind to address");
    let shutdown = ShutdownManager::new();

    match axum::serve(listener, app).await {
        Ok(()) => {
            shutdown.trigger_shutdown(0).ok();
        }
        Err(e) => {
            info!("server error: {}", e);
            shutdown.trigger_shutdown(1).ok();
        }
    };

    let exit_code = shutdown.wait_shutdown_complete().await;
    std::process::exit(exit_code);
}

fn create_blog_router(service: Arc<Service>) -> Router {
    let blog_routers = Router::new()
        .route("/", get(Handler::list_blogs).post(Handler::update_blog))
        .route("/{id}", get(Handler::get_blog))
        .route("/images", post(Handler::upload_blog_image))
        .route("/drafts", post(Handler::create_draft))
        .fallback(api_fallback)
        .with_state(service);

    Router::new().nest("/blogs", blog_routers)
}

fn create_trajectory_router(service: Arc<Service>) -> Router {
    let trajectory_routers = Router::new()
        .route("/upload", post(Handler::upload_gpx))
        .fallback(api_fallback)
        .with_state(service);

    Router::new().nest("/trajectory", trajectory_routers)
}

fn create_health_router(pool: PgPool) -> Router {
    Router::new()
        .route("/", get(health_ok))
        .route("/db", get(db_health_ok))
        .with_state(pool)
}

async fn initialize_db() -> PgPool {
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("Failed to connect to database");
    pool
}

async fn initialize_cloud_storage() -> Client {
    let account_id = env::var("CLOUDFLARE_ACCOUNT_ID").expect("CLOUDFLARE_ACCOUNT_ID must be set");
    let access_key_id =
        env::var("CLOUDFLARE_ACCESS_KEY_ID").expect("CLOUDFLARE_ACCESS_KEY_ID must be set");
    let secret_access_key =
        env::var("CLOUDFLARE_SECRET_ACCESS_KEY").expect("CLOUDFLARE_SECRET_ACCESS_KEY must be set");
    let config = aws_config::defaults(BehaviorVersion::latest())
        .endpoint_url(format!("https://{}.r2.cloudflarestorage.com", account_id))
        .region(Region::new("auto"))
        .credentials_provider(Credentials::new(
            access_key_id,
            secret_access_key,
            None,
            None,
            "R2",
        ))
        .load()
        .await;
    Client::new(&config)
}

async fn fallback() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "Not Found")
}

async fn health_ok() -> StatusCode {
    StatusCode::OK
}

async fn db_health_ok(State(pool): State<PgPool>) -> StatusCode {
    let connection_result = sqlx::query("SELECT 1").fetch_one(&pool).await;
    match connection_result {
        Ok(_) => StatusCode::OK,
        Err(_) => StatusCode::INTERNAL_SERVER_ERROR,
    }
}

async fn api_fallback() -> (StatusCode, Json<serde_json::Value>) {
    (
        StatusCode::NOT_FOUND,
        Json(serde_json::json!({
            "status": "Not Found"
        })),
    )
}
