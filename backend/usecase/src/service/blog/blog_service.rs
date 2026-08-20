use super::super::service::Service;
use crate::errors::app_error::AppError;
use crate::model::blog::{Blog, BlogFilter, BlogRequest, BlogStatus};
use crate::model::helper::uuid_from_string;
use crate::model::image::Image;

use async_trait::async_trait;
use bytes::Bytes;
use shared::markdown_converter::convert;
use tracing::error;
use uuid::Uuid;

#[async_trait]
pub trait BlogService {
    async fn get_blog(&self, id: String) -> Result<Blog, AppError>;
    async fn list_blogs(
        &self,
        year: Option<&String>,
        month: Option<&String>,
        status: Option<BlogStatus>,
        tag: Option<&String>,
    ) -> Vec<Blog>;
    async fn create_blog(&self, blog: BlogRequest) -> Result<Blog, AppError>;
    async fn update_blog(&self, blog: BlogRequest) -> Result<Blog, AppError>;
    async fn create_draft(&self) -> Result<String, AppError>;
    async fn upload_blog_image(&self, image_data: Bytes) -> Result<Image, AppError>;
}

#[async_trait]
impl BlogService for Service {
    async fn list_blogs(
        &self,
        year: Option<&String>,
        month: Option<&String>,
        status: Option<BlogStatus>,
        tag: Option<&String>,
    ) -> Vec<Blog> {
        let filter = BlogFilter::new(year, month, status, tag);
        let blogs = self.repository.list_blogs(filter).await;

        blogs
    }

    async fn get_blog(&self, id: String) -> Result<Blog, AppError> {
        let blog = self
            .repository
            .get_blog(uuid_from_string(&id)?)
            .await
            .map_err(|e| {
                error!("Failed to get blog by id: {}, err: {}", id, e);
                AppError::internal(Some("Failed to get blog"))
            })?;
        Ok(blog)
    }

    async fn create_draft(&self) -> Result<String, AppError> {
        let id = Uuid::now_v7();
        let mut initial_slug = id.to_string();
        initial_slug.truncate(25);
        let blog = Blog {
            id,
            title: Blog::default_title(),
            slug: initial_slug,
            content_key: format!("uploads/blogs/{}.html", id),
            status: BlogStatus::Draft,
            published_at: None,
            tag: None,
        };
        let mut tx = self.repository.create_transaction().await?;
        let id_str = self.repository.create_draft(&mut tx, blog).await?;
        tx.commit().await.map_err(|e| {
            error!("Failed to commit transaction for creating draft: {e}");
            AppError::internal(Some("Transaction commit failed"))
        })?;
        Ok(id_str)
    }

    async fn create_blog(&self, blog_req: BlogRequest) -> Result<Blog, AppError> {
        let uuid = Uuid::now_v7();
        let content_key = format!("uploads/blogs/{}.html", blog_req.id);

        let blog = Blog {
            id: uuid,
            title: Blog::default_title(),
            slug: "".to_string(),
            content_key: content_key,
            status: BlogStatus::Draft,
            published_at: None,
            tag: None,
        };

        let result = {
            let mut tx = self.repository.create_transaction().await?;
            let blog = self.repository.create_blog(&mut tx, blog).await?;
            self.repository
                .upload_blog_file(uuid.to_string(), blog_req.content)
                .await?;

            tx.commit().await.map_err(|e| {
                error!("Failed to commit transaction for creating blog: {e}");
                AppError::internal(Some("Transaction commit failed"))
            })?;

            Ok::<Blog, AppError>(blog)
        };

        result
    }

    async fn update_blog(&self, blog_req: BlogRequest) -> Result<Blog, AppError> {
        let blog_id = Uuid::parse_str(&blog_req.id.clone()).map_err(|_| {
            error!("A format of blog id is invalid");
            return AppError::invalid(Some("Invalid blog id"));
        })?;

        let mut blog = self.repository.get_blog(blog_id).await?;
        let content_key = format!("uploads/blogs/{}.html", blog_req.id);
        if blog.content_key != content_key {
            blog.content_key = content_key;
        }

        if let Some(title) = blog_req.title {
            blog.title = title;
        }

        if let Some(slug) = blog_req.slug {
            blog.slug = slug;
        }

        if let Some(status) = blog_req.status {
            blog.status = status;
        }

        if matches!(blog.status, BlogStatus::Published) && blog.published_at.is_none() {
            blog.published_at = Some(chrono::Utc::now().naive_utc());
        }

        if let Some(tag) = blog_req.tag {
            blog.tag = Some(tag);
        }

        let content_html = convert(&blog_req.content).map_err(|e| {
            error!("Failed to convert markdown into html");
            return AppError::internal(Some("Failed to convert markdown file"));
        })?;

        let result = {
            let mut tx = self.repository.create_transaction().await?;
            let blog = self.repository.update_blog(&mut tx, blog).await?;
            self.repository
                .upload_blog_file(blog_req.id, content_html)
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
        let image_id = Uuid::now_v7().to_string();
        self.repository
            .upload_image(image_id, image_data)
            .await
            .map_err(|e| {
                error!("Failed to upload blog image: {e}");
                AppError::internal(Some("Failed to upload blog image"))
            })
    }
}
