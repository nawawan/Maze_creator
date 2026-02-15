use uuid::Uuid;
#[derive(Clone)]
pub struct User {
    pub name: String,
    pub password: String,
    pub salt: Uuid
}