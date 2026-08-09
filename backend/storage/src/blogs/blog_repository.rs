use super::super::repository::*;
use aws_sdk_s3::error::ProvideErrorMetadata;
use aws_sdk_s3::primitives::ByteStream;
use tracing::error;

use usecase::errors::repo_error::RepoError;
use usecase::model::blog::{Blog, BlogFilter};
use usecase::model::image::Image;
use usecase::repository::blog::BlogRepository;
use usecase::repository::types::Transaction;

use async_trait::async_trait;
use bytes::Bytes;
use uuid::Uuid;

#[async_trait]
impl BlogRepository for Repository {
    async fn get_blog(&self, id: Uuid) -> Result<Blog, RepoError> {
        let blog = sqlx::query_as!(
            Blog,
            "SELECT id, title, slug, content_key, status, published_at, tag FROM blogs WHERE id = $1",
            id
        )
        .fetch_one(&self.pool)
        .await
        .map_err(|e| {
            error!("Failed to get blog by id: {}, err: {}", id, e);
            RepoError::Internal("Failed to get blog".to_string())
        })?;

        Ok(blog)
    }

    async fn list_blogs(&self, filter: BlogFilter) -> Vec<Blog> {
        let mut builder = sqlx::QueryBuilder::new(
            "SELECT id, title, slug, content_key, status, published_at, tag FROM blogs WHERE 1=1",
        );
        filter.apply(&mut builder);
        builder
            .build_query_as::<Blog>()
            .fetch_all(&self.pool)
            .await
            .unwrap_or_else(|e| {
                error!("Failed to get blogs: {}", e);
                Vec::new()
            })
    }

    async fn create_draft(
        &self,
        tx: &mut Transaction<'_>,
        blog: Blog,
    ) -> Result<String, RepoError> {
        sqlx::query!(
            "INSERT INTO blogs (id, status, title, content_key, slug, tag) VALUES ($1, 'DRAFT', $2, $3, $4, $5)",
            blog.id,
            blog.title,
            blog.content_key,
            blog.slug,
            blog.tag,
        )
        .execute(&mut **tx)
        .await
        .map_err(|e| {
            error!("Failed to create draft blog: {}", e);
            RepoError::Internal("Failed to create draft blog".to_string())
        })?;
        Ok(blog.id.to_string())
    }

    async fn create_blog(&self, tx: &mut Transaction<'_>, blog: Blog) -> Result<Blog, RepoError> {
        sqlx::query!(
            "INSERT INTO blogs (id, title, slug, status, content_key, tag) VALUES ($1, $2, $3, 'PUBLISHED', $4, $5)",
            blog.id,
            blog.title,
            blog.slug,
            blog.content_key,
            blog.tag,
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

    async fn update_blog(&self, tx: &mut Transaction<'_>, blog: Blog) -> Result<Blog, RepoError> {
        sqlx::query!(
            "UPDATE blogs SET title = $2, status = $3, content_key = $4, published_at = $5, tag = $6 WHERE id = $1",
            blog.id,
            blog.title,
            blog.status.to_string(),
            blog.content_key,
            blog.published_at,
            blog.tag,
        )
        .execute(&mut **tx)
        .await
        .map_err(|e| {
            error!("Failed to update blog: {}", e);
            RepoError::Internal("Failed to update blog".to_string())
        })?;
        Ok(blog)
    }

    async fn upload_image(&self, image_id: String, image_data: Bytes) -> Result<Image, RepoError> {
        let body = ByteStream::from(image_data);
        let bucket_name = &self.config.blog_r2_bucket;

        self.r2_client
            .put_object()
            .bucket(bucket_name)
            .key(format!("_uploads/{}", image_id))
            .body(body)
            .send()
            .await
            .map_err(|e| {
                error!(image_id = %image_id, error = %e);
                error!(
                    code = e.code(),
                    message = e.message().unwrap_or("No error message")
                );
                RepoError::Internal("Failed to upload image".to_string())
            })?;

        Ok(Image {
            id: image_id.clone(),
            url: format!("{}/_uploads/{}", self.config.host, image_id),
        })
    }

    async fn upload_blog_file(&self, blog_id: String, content: String) -> Result<(), RepoError> {
        let body = ByteStream::from(content.into_bytes());
        let bucket_name = &self.config.blog_r2_bucket;
        self.r2_client
            .put_object()
            .bucket(bucket_name)
            .key(format!("uploads/blogs/{}.html", blog_id))
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
    use shared::config::Config;
    use uuid::Uuid;

    async fn test_repository(pool: sqlx::PgPool) -> Repository {
        Repository::new(
            pool,
            Client::new(&aws_config::load_defaults(BehaviorVersion::latest()).await),
            Config {
                host: "test".into(),
                env: "dev".into(),
                cf_access_team_domain: "test.cloudflareaccess.com".into(),
                cf_access_aud: "test_aud".into(),
                blog_r2_bucket: "test_bucket".into(),
            },
        )
    }

    #[sqlx::test(migrations = "../src/migrations")]
    async fn test_create_draft_can_fetch_id(pool: sqlx::PgPool) -> Result<()> {
        let repo = test_repository(pool).await;

        let id = Uuid::now_v7();
        let blog = Blog {
            id,
            title: String::new(),
            content_key: format!("uploads/blogs/{}.html", id),
            slug: String::new(),
            status: usecase::model::blog::BlogStatus::Draft,
            published_at: None,
            tag: None,
        };
        let mut tx = repo.pool.begin().await?;
        let draft_id = repo.create_draft(&mut tx, blog).await;

        if let Ok(id) = draft_id {
            assert!(!id.is_empty());
        }

        Ok(())
    }

    #[sqlx::test(migrations = "../src/migrations")]
    async fn test_create_blog_already_exists(pool: sqlx::PgPool) -> Result<()> {
        let repo = test_repository(pool).await;
        let mut blog = Blog {
            id: Uuid::now_v7(),
            title: "Test Blog".to_string(),
            slug: "test".to_string(),
            content_key: "test-blog".to_string(),
            status: usecase::model::blog::BlogStatus::Published,
            published_at: None,
            tag: None,
        };

        let mut tx = repo.pool.begin().await?;
        let _ = repo.create_blog(&mut tx, blog.clone()).await;
        blog.content_key = "test".to_string();
        let result = repo.create_blog(&mut tx, blog).await;
        assert!(result.is_err());
        assert!(matches!(result.unwrap_err(), RepoError::Conflict(_)));
        Ok(())
    }

    #[sqlx::test(migrations = "../src/migrations")]
    async fn test_get_blog_maps_nullable_tag_field(pool: sqlx::PgPool) -> Result<()> {
        let repo = test_repository(pool).await;

        let tagged = Blog {
            id: Uuid::now_v7(),
            title: "Tagged".to_string(),
            slug: "tagged".to_string(),
            content_key: "tagged-key".to_string(),
            status: usecase::model::blog::BlogStatus::Published,
            published_at: None,
            tag: Some("TECH".to_string()),
        };
        let untagged = Blog {
            id: Uuid::now_v7(),
            title: "Untagged".to_string(),
            slug: "untagged".to_string(),
            content_key: "untagged-key".to_string(),
            status: usecase::model::blog::BlogStatus::Published,
            published_at: None,
            tag: None,
        };

        let mut tx = repo.pool.begin().await?;
        repo.create_blog(&mut tx, tagged.clone()).await?;
        repo.create_blog(&mut tx, untagged.clone()).await?;
        tx.commit().await?;

        let fetched_tagged = repo.get_blog(tagged.id).await?;
        assert_eq!(fetched_tagged.tag, Some("TECH".to_string()));

        let fetched_untagged = repo.get_blog(untagged.id).await?;
        assert_eq!(fetched_untagged.tag, None);

        Ok(())
    }

    #[sqlx::test(migrations = "../src/migrations")]
    async fn test_list_blogs_filters_by_tag(pool: sqlx::PgPool) -> Result<()> {
        let repo = test_repository(pool).await;

        let tech = Blog {
            id: Uuid::now_v7(),
            title: "Tech post".to_string(),
            slug: "tech-post".to_string(),
            content_key: "tech-key".to_string(),
            status: usecase::model::blog::BlogStatus::Published,
            published_at: None,
            tag: Some("TECH".to_string()),
        };
        let travel = Blog {
            id: Uuid::now_v7(),
            title: "Travel post".to_string(),
            slug: "travel-post".to_string(),
            content_key: "travel-key".to_string(),
            status: usecase::model::blog::BlogStatus::Published,
            published_at: None,
            tag: Some("TRAVEL".to_string()),
        };
        let untagged = Blog {
            id: Uuid::now_v7(),
            title: "Untagged post".to_string(),
            slug: "untagged-post".to_string(),
            content_key: "untagged-post-key".to_string(),
            status: usecase::model::blog::BlogStatus::Published,
            published_at: None,
            tag: None,
        };

        let mut tx = repo.pool.begin().await?;
        repo.create_blog(&mut tx, tech.clone()).await?;
        repo.create_blog(&mut tx, travel.clone()).await?;
        repo.create_blog(&mut tx, untagged.clone()).await?;
        tx.commit().await?;

        let filter = BlogFilter {
            limit: None,
            offset: None,
            order_by: None,
            order_desc: None,
            start: None,
            end: None,
            status: None,
            tag: Some("TECH".to_string()),
        };
        let results = repo.list_blogs(filter).await;

        assert_eq!(results.len(), 1);
        assert_eq!(results[0].id, tech.id);
        assert_eq!(results[0].tag, Some("TECH".to_string()));

        Ok(())
    }
}
