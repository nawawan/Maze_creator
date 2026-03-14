use super::super::repository::*;
use aws_sdk_s3::primitives::ByteStream;
use tracing::error;
use usecase::errors::repo_error::RepoError;
use usecase::model::blog::{Blog, BlogFilter};
use usecase::model::image::Image;
use usecase::repository::blog::BlogRepository;
use usecase::repository::types::Transaction;

use async_trait::async_trait;
use bytes::Bytes;

#[async_trait]
impl BlogRepository for Repository {
    async fn get_blogs(&self, filter: BlogFilter) -> Vec<Blog> {
        // let rows = self.client.query("SELECT * FROM blogs", &[]).await.unwrap();
        return vec![];
    }

    async fn create_draft(&self, tx: &mut Transaction<'_>) -> Result<String, RepoError> {
        let res =
            sqlx::query!("INSERT INTO blogs (id, status) VALUES (DEFAULT, 'DRAFT') RETURNING id")
                .fetch_one(&mut **tx)
                .await
                .map_err(|e| {
                    error!("Failed to create draft blog: {}", e);
                    RepoError::Internal("Failed to create draft blog".to_string())
                })?;
        return Ok(res.id.simple().to_string());
    }

    async fn create_blog(&self, tx: &mut Transaction<'_>, blog: Blog) -> Result<Blog, RepoError> {
        sqlx::query!(
            "INSERT INTO blogs (id, title, status, content_key) VALUES ($1, $2, 'PUBLISHED', $3)",
            blog.id,
            blog.title,
            blog.content_key
        )
        .execute(&mut **tx)
        .await
        .map_err(|e| {
            if let Some(db_err) = e.as_database_error() {
                if db_err.code() == Some("23505".into()) {
                    error!(
                        "Blog with the same id: {} already exists, err: {}",
                        blog.id, e
                    );
                    return RepoError::Conflict(format!("Blog with the same id already exists"));
                }
            }
            error!("Failed to create blog: {}", e);
            RepoError::Internal("Failed to create blog".to_string())
        })?;
        Ok(blog)
    }

    async fn upload_image(&self, image_id: String, image_data: Bytes) -> Result<Image, RepoError> {
        let body = ByteStream::from(image_data);
        let bucket_name = "blog-assets/_uploads";

        self.r2_client
            .put_object()
            .bucket(bucket_name)
            .key(format!("{}", image_id))
            .body(body)
            .send()
            .await
            .map_err(|e| {
                error!("Failed to upload image id: {} err: {}", image_id, e);
                RepoError::Internal("Failed to upload image".to_string())
            })?;
            
        Ok(Image {
            id: image_id.clone(),
            url: format!("images/{}", image_id),
        })
    }

    async fn upload_blog_draft(&self, blog_id: String, content: String) -> Result<(), RepoError> {
        let body = ByteStream::from(content.into_bytes());
        let bucket_name = "blog-assets/uploads";
        self.r2_client
            .put_object()
            .bucket(bucket_name)
            .key(format!("drafts/{}", blog_id))
            .body(body)
            .send()
            .await
            .map_err(|e| {
                error!("Failed to upload blog draft, id: {} err : {}", blog_id, e);
                RepoError::Internal("Failed to upload blog draft".to_string())
            })?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use anyhow::Result;
    use aws_config::BehaviorVersion;
    use aws_sdk_s3::Client;
    use uuid::Uuid;

    #[sqlx::test(migrations = "../src/migrations")]
    async fn test_create_draft_can_fetch_id(pool: sqlx::PgPool) -> Result<()> {
        let repo = Repository::new(
            pool,
            Client::new(&aws_config::load_defaults(BehaviorVersion::latest()).await),
        );

        let mut tx = repo.pool.begin().await?;
        let draft_id = repo.create_draft(&mut tx).await;

        if let Ok(id) = draft_id {
            assert!(!id.is_empty());
        }

        Ok(())
    }

    #[sqlx::test(migrations = "../src/migrations")]
    async fn test_create_blog_already_exists(pool: sqlx::PgPool) -> Result<()> {
        let repo = Repository::new(
            pool,
            Client::new(&aws_config::load_defaults(BehaviorVersion::latest()).await),
        );
        let blog = Blog {
            id: Uuid::now_v7(),
            title: "Test Blog".to_string(),
            content_key: "test-blog".to_string(),
            status: usecase::model::blog::BlogStatus::Published,
        };

        let mut tx = repo.pool.begin().await?;
        let _ = repo.create_blog(&mut tx, blog.clone()).await;
        let result = repo.create_blog(&mut tx, blog).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), RepoError::Conflict(_)));
        Ok(())
    }
}
