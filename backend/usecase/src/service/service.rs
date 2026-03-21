use crate::repository::repositories::Repositories;
use shared::config::Config;
pub struct Service {
    pub config: Config,
    pub repository: Box<dyn Repositories>,
}

impl Service {
    pub fn new(config: Config, repository: Box<dyn Repositories>) -> Self {
        Self { config, repository }
    }
}
