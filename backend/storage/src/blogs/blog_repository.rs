use usecase::repository::blog::{BlogRepository};
use usecase::model::blog::{Blog, BlogFilter};
use usecase::errors::repo_error::RepoError;
use super::super::repository::*;
use aws_sdk_s3::primitives::ByteStream;
use tracing::error;

use async_trait::async_trait;

#[async_trait]
impl BlogRepository for Repository {
    async fn get_blogs(&self, filter: BlogFilter) -> Vec<Blog> {

        // let rows = self.client.query("SELECT * FROM blogs", &[]).await.unwrap();
        return vec![];
    }

    async fn create_draft(&self) -> Result<String, RepoError> {
        let res= sqlx::query!("INSERT INTO blogs (id, status) VALUES (DEFAULT, 'DRAFT') RETURNING id").fetch_one(&self.pool).await.map_err(|e| {
            error!("Failed to create draft blog: {}", e);
            RepoError::Internal("Failed to create draft blog".to_string())
        })?;
        return Ok(res.id.simple().to_string());
    }

    async fn create_blog(&self, blog: Blog) -> Result<(), RepoError> {
        sqlx::query!("INSERT INTO blogs (id, title, status) VALUES ($1, $2, 'PUBLISHED')", blog.id, blog.title).execute(&self.pool).await.map_err(|e| {
            if let Some(db_err) = e.as_database_error() {
                if db_err.code() == Some("23505".into()) {
                    error!("Blog with the same id: {} already exists, err: {}", blog.id, e);
                    return RepoError::Conflict(format!("Blog with the same id already exists"));
                }
            }
            error!("Failed to create blog: {}", e);
            RepoError::Internal("Failed to create blog".to_string())
        })?;
        Ok(())
    }

    async fn upload_image(&self, image_id: String, image_data: Vec<u8>) -> Result<(), RepoError> {
        // Implement the logic to upload the image and return the URL
        let body = ByteStream::from(image_data);
        let bucket_name = "blog-assets/_uploads";

        self.r2_client.put_object().bucket(bucket_name).key(format!("{}", image_id)).body(body).send().await.map_err(|e| {
            error!("Failed to upload image id: {} err: {}", image_id, e);
            RepoError::Internal("Failed to upload image".to_string())
        })?;
        Ok(())
    }

    async fn upload_blog_draft(&self, blog_id: String, content: String) -> Result<(), RepoError> {
        // Implement the logic to upload the blog draft content
        let body = ByteStream::from(content.into_bytes());
        let bucket_name = "blog-assets/uploads";
        self.r2_client.put_object().bucket(bucket_name).key(format!("drafts/{}", blog_id)).body(body).send().await.map_err(|e| {
            error!("Failed to upload blog draft, id: {} err : {}", blog_id, e);
            RepoError::Internal("Failed to upload blog draft".to_string())
        })?;
        Ok(())
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    use aws_config::BehaviorVersion;
    use aws_sdk_s3::Client;
    use anyhow::Result;

    #[sqlx::test]
    async fn test_create_draft_can_fetch_id(pool: sqlx::PgPool) -> Result<()> {
        let repo = Repository::new(pool, Client::new(&aws_config::load_defaults(BehaviorVersion::latest()).await));

        let draft_id = repo.create_draft().await;

        if let Ok(id) = draft_id {
            assert!(!id.is_empty());
        }

        Ok(())
    }
}