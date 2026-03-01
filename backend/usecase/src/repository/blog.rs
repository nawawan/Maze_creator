use super::super::model::blog::*;
use async_trait::async_trait;
use anyhow::Result;

#[async_trait]
pub trait BlogRepository: Send + Sync {
    async fn get_blogs(&self, filter: BlogFilter) -> Vec<Blog>;
    async fn create_draft(&self) -> Result<String>;
    async fn create_blog(&self, blog: Blog) -> Result<()>;
    async fn upload_image(&self, blog_id: String, image_data: Vec<u8>) -> Result<()>;
    async fn upload_blog_draft(&self, blog_id: String, content: String) -> Result<()>;
}