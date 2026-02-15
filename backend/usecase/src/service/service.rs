use crate::repository::repositories::Reporisories;
pub struct Service {
    pub repository: Box<dyn Reporisories>,
}

impl Service {
    pub fn new(repository: Box<dyn Reporisories>) -> Self {
        Self { repository }
    }
}