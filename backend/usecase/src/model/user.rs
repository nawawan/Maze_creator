use uuid::Uuid;
#[derive(Clone)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub password: String,
    pub salt: String,
}

#[derive(Clone)]
pub struct Token {
    pub id: Uuid,
    pub access_token: String,
}

impl Token {
    pub fn new(user_id: Uuid) -> Self {
        Self {
            id: user_id,
            access_token: Uuid::now_v7().simple().to_string(),
        }
    }
}
