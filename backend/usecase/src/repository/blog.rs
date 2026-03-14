use bytes::Bytes;

use crate::model::image::Image;

use super::super::model::blog::*;
use async_trait::async_trait;
use super::super::errors::repo_error::RepoError;
use super::types::Transaction;

#[async_trait]
pub trait BlogRepository: Send + Sync {
    async fn get_blogs(&self, filter: BlogFilter) -> Vec<Blog>;
    async fn create_draft(&self, tx: &mut Transaction<'_>) -> Result<String, RepoError>;
    async fn create_blog(&self, tx: &mut Transaction<'_>, blog: Blog) -> Result<Blog, RepoError>;
    async fn upload_image(&self, blog_id: String, image_data: Bytes) -> Result<Image, RepoError>;
    async fn upload_blog_draft(&self, blog_id: String, content: String) -> Result<(), RepoError>;
}