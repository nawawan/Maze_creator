pub mod model;

use redis::{AsyncCommands, Client};
use shared::config::RedisConfig;
use usecase::errors::app_error::AppError;

use self::model::{RedisKey, RedisValue};

pub struct RedisClient {
    client: Client,
}

impl RedisClient {
    pub fn new(config: RedisConfig) -> Result<Self, AppError> {
        let client = Client::open(format!("redis://{}:{}", config.host, config.port))
            .map_err(|e| {
                return AppError::internal(e.detail())
            })?;
        Ok(Self { client })
    }

    pub async fn insert(&self, key: String, val: String) -> Result<(), AppError>{
        let mut con = self.client.get_multiplexed_async_connection().await?;
        let _: () = con.set(key, val).await?;
        Ok(())
    }

    pub async fn get(&self, key: String) -> Result<String, AppError> {
        let mut con = self.client.get_multiplexed_async_connection().await?;
        let res: String = con.get(key).await?;
        Ok(res)
    }
}