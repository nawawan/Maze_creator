pub struct DatabaseConfig {
    pub url: String,
    pub max_connection: u32,
}

pub struct StorageConfig {
    pub blog_bucket: String,
    pub blog_image_bucket: String,
}

pub struct Config {
    pub host: String
}

pub struct RedisConfig {
    pub host: String,
    pub port: u16,
}

impl DatabaseConfig {
    pub fn new(url: String, max_connection: u32) -> Self {
        Self {
            url,
            max_connection,
        }
    }
}
