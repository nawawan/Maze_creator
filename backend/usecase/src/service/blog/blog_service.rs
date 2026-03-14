use crate::errors::app_error::AppError;
use crate::model::image::Image;
use crate::model::blog::{Blog, BlogFilter, BlogRequest, BlogStatus};

use super::super::service::Service;
use async_trait::async_trait;
use std::env;
use tracing::error;
use bytes::Bytes;
use uuid::Uuid;

#[async_trait]
pub trait BlogService {
    fn get_blogs(&self, year: Option<&String>, month: Option<&String>);
    async fn create_blog(&self, blog: BlogRequest) -> Result<Blog, AppError>;
    async fn create_draft(&self) -> Result<String, AppError>;
    async fn upload_blog_image(&self, image_data: Bytes) -> Result<Image, AppError>;
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

    async fn create_blog(&self, blog_req: BlogRequest) -> Result<Blog, AppError> {
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
        };

        let result = {
            let mut tx = self.repository.create_transaction().await?;
            let blog = self.repository.create_blog(&mut tx, blog).await?;
            self.repository
                .upload_blog_draft(uuid.to_string(), blog_req.content)
                .await?;

            tx.commit().await.map_err(|e| {
                error!("Failed to commit transaction for creating blog: {e}");
                AppError::internal(Some("Transaction commit failed"))
            })?;

            Ok::<Blog, AppError>(blog)
        };

        result
    }


    async fn upload_blog_image(&self, image_data: Bytes) -> Result<Image, AppError> {
        let image_id = Uuid::now_v7().to_string().replace("-", "");
        self.repository
            .upload_image(image_id, image_data)
            .await
            .map_err(|e| {
                error!("Failed to upload blog image: {e}");
                AppError::internal(Some("Failed to upload blog image"))
            })
    }
}
