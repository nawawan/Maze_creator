use sqlx::PgPool;

use shared::config::DatabaseConfig;
use sqlx::postgres::{PgPoolOptions};

#[derive(Clone)]
pub struct ConnectionPool {
    pub pool: PgPool,
}

impl ConnectionPool {
    pub fn inner_ref(&self) -> &PgPool {
        &self.pool
    }
}

pub async fn connect_database(config: &DatabaseConfig) -> ConnectionPool {
    let pool = PgPoolOptions::new().max_connections(config.max_connection).connect(&config.url).await.expect("Failed to connect to database");

    ConnectionPool { pool }
}