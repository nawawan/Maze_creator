use usecase::errors::repo_error::RepoError;

pub trait RedisKey {
    type Value : RedisValue + TryFrom<String, Error=RepoError>;
    fn inner(&self) -> String;
}

pub trait RedisValue {
    fn inner(&self) -> String;
}