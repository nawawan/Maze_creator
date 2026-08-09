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
    pub cf_access_team_domain: String,
    pub cf_access_aud: String,
    pub blog_r2_bucket: String,
}

impl DatabaseConfig {
    pub fn new(url: String, max_connection: u32) -> Self {
        Self {
            url,
            max_connection,
        }
    }
}
