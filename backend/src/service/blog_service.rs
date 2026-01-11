use super::service::{Service};

use super::super::model::blog::BlogFilter;
use super::super::repository::blog_repository::*;

pub trait BlogService {
    fn create_blog(&self);
    fn get_blogs(&self, year: Option<&String>, month: Option<&String>);
}

impl BlogService for Service {
    fn create_blog(&self) {
        
    }

    fn get_blogs(&self, year: Option<&String>, month: Option<&String>) {
        if year.is_none() && !month.is_none() {
            return;
        }
        
        let filter = BlogFilter::new(year, month);

        let blogs = self.repository.get_blogs(filter);
    }
}