#[derive(Clone)]
pub struct User {
    pub id: String,
    pub name: String,
    pub password: String,
    pub salt: String,
}
