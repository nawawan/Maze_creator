use usecase::errors::app_error::AppError;

pub trait RedisKey {
    type Value : RedisValue + TryFrom<String, Error=AppError>;
    fn inner(&self) -> String;
}

pub trait RedisValue {
    fn inner(&self) -> String;
}