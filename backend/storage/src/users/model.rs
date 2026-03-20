use crate::redis::model::RedisValue;

use super::super::redis::model::RedisKey;
use usecase::{errors::app_error::AppError, model::user::Token};

pub struct AccessToken(pub String);
pub struct AuthorizedUserId(pub String);


impl From<Token> for AccessToken {
    fn from(value: Token) -> Self {
        Self(value.access_token)
    }
}

impl RedisKey for AccessToken {
    type Value = AuthorizedUserId;
    fn inner(&self) -> String {
        self.0.clone()
    }
}

impl RedisValue for AuthorizedUserId {
    fn inner(&self) -> String {
        self.0.clone()
    }
}

impl TryFrom<String> for AuthorizedUserId {
    type Error = AppError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(value.clone()))
    }
}