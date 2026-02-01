
pub struct DatabaseConfig {
    pub url: String,
    pub max_connection: u32,
}

impl DatabaseConfig {
    pub fn new(url: String, max_connection: u32) -> Self {
        Self { url, max_connection }
    }
}