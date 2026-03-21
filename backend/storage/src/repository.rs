use crate::redis::RedisClient;
use aws_sdk_s3::Client;
use shared::config::Config;
use sqlx::PgPool;
use usecase::repository::repositories::Repositories;

pub struct Repository {
    pub pool: PgPool,
    pub r2_client: Client,
    pub redis_client: RedisClient,
    pub config: Config,
}

impl Repository {
    pub fn new(pool: PgPool, r2_client: Client, redis_client: RedisClient, config: Config) -> Self {
        Self {
            pool,
            r2_client,
            redis_client,
            config,
        }
    }
}

impl Repositories for Repository {}
