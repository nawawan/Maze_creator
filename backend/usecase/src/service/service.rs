use super::super::repository::blog::BlogRepository;
use super::super::repository::user::UserRepository;

pub trait Reporisories: BlogRepository + UserRepository {}
pub struct Service {
    pub repository: Box<dyn Reporisories>,
}

impl Service {
    pub fn new(repository: Box<dyn Reporisories>) -> Self {
        Self { repository }
    }
}