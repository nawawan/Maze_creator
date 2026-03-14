use aws_sdk_s3::Client;
use sqlx::PgPool;
use usecase::repository::repositories::Repositories;
use shared::config::Config;

pub struct Repository {
    pub pool: PgPool,
    pub r2_client: Client,
    pub config: Config,
}

impl Repository {
    pub fn new(pool: PgPool, r2_client: Client, config: Config) -> Self {
        Self { pool, r2_client, config }
    }
}

impl Repositories for Repository {}
