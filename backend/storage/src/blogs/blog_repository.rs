use usecase::repository::blog::{BlogRepository};
use usecase::model::blog::{Blog, BlogFilter};
use super::super::repository::*;
use anyhow::Result;
use aws_sdk_s3::primitives::ByteStream;

use async_trait::async_trait;

#[async_trait]
impl BlogRepository for Repository {
    async fn get_blogs(&self, filter: BlogFilter) -> Vec<Blog> {

        // let rows = self.client.query("SELECT * FROM blogs", &[]).await.unwrap();
        return vec![];
    }

    async fn create_draft(&self) -> Result<String> {
       let res= sqlx::query!("INSERT INTO blogs (id, status) VALUES (DEFAULT, 'DRAFT') RETURNING id").fetch_one(&self.pool).await?;
       return Ok(res.id.simple().to_string());
    }

    async fn create_blog(&self, blog: Blog) -> Result<()> {
        sqlx::query!("INSERT INTO blogs (id, title, status) VALUES ($1, $2, 'PUBLISHED')", blog.id, blog.title).execute(&self.pool).await?;
        Ok(())
    }

    async fn upload_image(&self, image_id: String, image_data: Vec<u8>) -> Result<()> {
        // Implement the logic to upload the image and return the URL
        let body = ByteStream::from(image_data);
        let bucket_name = "blog-assets/_uploads";

        let resp = self.r2_client.put_object().bucket(bucket_name).key(format!("{}", image_id)).body(body).send().await?;
        Ok(())
    }

    async fn upload_blog_draft(&self, blog_id: String, content: String) -> Result<()> {
        // Implement the logic to upload the blog draft content
        let body = ByteStream::from(content.into_bytes());
        let bucket_name = "blog-assets/uploads";
        self.r2_client.put_object().bucket(bucket_name).key(format!("drafts/{}", blog_id)).body(body).send().await?;
        Ok(())
    }
}


#[cfg(test)]
mod tests {

    use super::*;
    use aws_config::BehaviorVersion;
    use aws_sdk_s3::Client;

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