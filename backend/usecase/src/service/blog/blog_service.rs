
use crate::errors::app_error::AppError;

use super::super::service::{Service};

use super::super::super::model::blog::{Blog, BlogFilter, BlogRequest, BlogStatus};
use async_trait::async_trait;
use anyhow::{anyhow, Result};
use tracing::error;
use uuid::{Uuid};
use std::env;

#[async_trait]
pub trait BlogService {
    fn get_blogs(&self, year: Option<&String>, month: Option<&String>);
    async fn create_blog(&self, blog: BlogRequest) -> Result<(), AppError>;
    async fn create_draft(&self) -> Result<String, AppError>;
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

    async fn create_draft(&self) -> Result<String, AppError> {
        let mut tx = self.repository.create_transaction().await?;
        let id = self.repository.create_draft(&mut tx).await?;
        tx.commit().await.map_err(|e| {
            error!("Failed to commit transaction for creating draft: {e}");
            AppError::internal(Some("Transaction commit failed"))
        })?;
        Ok(id)
    }

    async fn create_blog(&self, blog_req: BlogRequest) -> Result<(), AppError> {
        let uuid = Uuid::now_v7();
        let blog_url = env::var("BLOG_PAGE");

        if let Err(e) = blog_url {
            error!("BLOG_PAGE environment variable is not set: {e}");
            return Err(AppError::internal(Some("environment variable is not set")));
        }
        let content_key = format!("{}/{}", blog_url.unwrap(), blog_req.title);

        let blog = Blog {
            id: uuid, 
            title: blog_req.title,
            content_key: content_key,
            status: BlogStatus::Published,
            created_at: chrono::Utc::now(),
            updated_at: chrono::Utc::now(),
        };

        let result = {
            let mut tx = self.repository.create_transaction().await?;
            self.repository.create_blog(&mut tx, blog).await?;
            self.repository.upload_blog_draft(uuid.to_string(), blog_req.content).await?;

            tx.commit().await.map_err(|e| {
                error!("Failed to commit transaction for creating blog: {e}");
                AppError::internal(Some("Transaction commit failed"))
            })?;

            Ok::<_, AppError>(())
        };

        result
    }
}