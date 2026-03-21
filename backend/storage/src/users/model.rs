use crate::redis::model::RedisValue;

use super::super::redis::model::RedisKey;
use usecase::{errors::repo_error::RepoError, model::user::Token};
use uuid::Uuid;

pub struct AccessToken(pub String);
pub struct AuthorizedUserId(pub String);

impl From<Token> for AccessToken {
    fn from(value: Token) -> Self {
        Self(value.access_token)
    }
}

impl From<String> for AccessToken {
    fn from(value: String) -> Self {
        Self(value)
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

impl From<Uuid> for AuthorizedUserId {
    fn from(value: Uuid) -> Self {
        Self(value.to_string())
    }
}

impl TryFrom<String> for AuthorizedUserId {
    type Error = RepoError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        Ok(Self(value.clone()))
    }
}
