use super::super::service::{Service};

use super::super::super::model::blog::{Blog, BlogFilter};
use async_trait::async_trait;
use anyhow::{Result};
use tracing::error;

#[async_trait]
pub trait BlogService {
    fn get_blogs(&self, year: Option<&String>, month: Option<&String>);
    fn create_blog(&self, blog: Blog);
    async fn create_draft(&self) -> Result<String>;
}

#[async_trait]
impl BlogService for Service {
    fn get_blogs(&self, year: Option<&String>, month: Option<&String>) {
        if year.is_none() && !month.is_none() {
            return;
        }
        
        let filter = BlogFilter::new(year, month);

        let blogs = self.repository.get_blogs(filter);
    }

    async fn create_draft(&self) -> Result<String> {
        self.repository.create_draft().await.map_err(|e| {
            error!("failed to create id for draft: {e}");
            anyhow::anyhow!("Failed to create draft")
        })
    }

    fn create_blog(&self, blog: Blog) {
        self.repository.create_draft();
    }
}