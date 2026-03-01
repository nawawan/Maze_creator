use usecase::model::user::User;

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize)]
pub struct UserResponse {
    pub id: String,
    pub username: String,
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id.to_string(),
            username: user.name,
        }
    }
}
