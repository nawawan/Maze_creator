use super::super::model::blog::*;
use async_trait::async_trait;
use super::super::errors::repo_error::RepoError;

#[async_trait]
pub trait BlogRepository: Send + Sync {
    async fn get_blogs(&self, filter: BlogFilter) -> Vec<Blog>;
    async fn create_draft(&self) -> Result<String, RepoError>;
    async fn create_blog(&self, blog: Blog) -> Result<(), RepoError>;
    async fn upload_image(&self, blog_id: String, image_data: Vec<u8>) -> Result<(), RepoError>;
    async fn upload_blog_draft(&self, blog_id: String, content: String) -> Result<(), RepoError>;
}