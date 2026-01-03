pub struct Service {
}

pub trait BlogService {
    fn create_blog(&self);
}

impl Service {
    pub fn new() -> Self {
        Self {}
    }
}