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

    pub async fn set_ex<T: RedisKey>(&self, key: T, val: &T::Value, ttl: u64) -> Result<(), AppError>{
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let _: () = conn.set_ex(key.inner(), val.inner(), ttl).await?;
        Ok(())
    }

    pub async fn get<T: RedisKey>(&self, key: T) -> Result<Option<T::Value>, AppError> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let res: Option<String> = conn.get(key.inner()).await?;
        res.map(T::Value::try_from).transpose()
    }

    pub async fn delete<T: RedisKey>(&self, key: T) -> Result<(), AppError> {
        let mut conn = self.client.get_multiplexed_async_connection().await?;
        let _: () = conn.del(key.inner()).await?;
        Ok(())
    }
}