pub struct DatabaseConfig {
    pub url: String,
    pub max_connection: u32,
}

pub struct StorageConfig {
    pub blog_bucket: String,
    pub blog_image_bucket: String,
}

#[derive(Clone)]
pub struct Config {
    pub host: String,
    pub env: String,
    pub token_ttl: u64,
    pub refresh_ttl: u64,
}

pub struct RedisConfig {
    pub host: String,
    pub port: String,
}

impl DatabaseConfig {
    pub fn new(url: String, max_connection: u32) -> Self {
        Self {
            url,
            max_connection,
        }
    }
}
