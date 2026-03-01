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