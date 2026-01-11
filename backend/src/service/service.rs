use crate::repository::repository::Repository;

pub struct Service {
    pub repository: Repository
}

impl Service {
    pub fn new(repository: Repository) -> Self {
        Self { repository }
    }
}