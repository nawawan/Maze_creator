use super::super::repository::blog::BlogRepository;

pub struct Service {
    pub repository: Box<dyn BlogRepository>
}

impl Service {
    pub fn new(repository: Box<dyn BlogRepository>) -> Self {
        Self { repository }
    }
}