use sqlx::PgPool;
use usecase::repository::repositories::Repositories;
use aws_sdk_s3::Client;

pub struct Repository {
    pub pool: PgPool,
    pub r2_client: Client,
}

impl Repository {
    pub fn new(pool: PgPool, r2_client: Client) -> Self {
        Self { pool, r2_client }
    }
}

impl Repositories for Repository {}