use crate::model::blog;

use super::super::service::{Service};

use super::super::super::model::blog::{Blog, BlogFilter, BlogRequest, BlogStatus};
use async_trait::async_trait;
use anyhow::{Result};
use tracing::error;
use uuid::{Uuid};
use std::env;

#[async_trait]
pub trait BlogService {
    fn get_blogs(&self, year: Option<&String>, month: Option<&String>);
    async fn create_blog(&self, blog: BlogRequest);
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

    async fn create_blog(&self, blog_req: BlogRequest) {
        let uuid = Uuid::now_v7();
        let blog_url = env::var("BLOG_PAGE");

        if let Err(e) = blog_url {
            error!("BLOG_PAGE environment variable is not set: {e}");
            return;
        }
        let content_key = format!("{}/{}", blog_url.unwrap(), blog_req.title);
        let contents = blog_req.content.as_bytes().to_vec();

        let blog = Blog {
            id: uuid, 
            title: blog_req.title,
            content_key: content_key,
            status: BlogStatus::Published,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };
        self.repository.create_blog(blog).await;
    }
}