use crate::repository::repositories::Repositories;
pub struct Service {
    pub repository: Box<dyn Repositories>,
}

impl Service {
    pub fn new(repository: Box<dyn Repositories>) -> Self {
        Self { repository }
    }
}
