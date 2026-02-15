use usecase::repository::blog::{BlogRepository};
use usecase::model::blog::{Blog, BlogFilter};
use super::super::repository::*;
use anyhow::Result;

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

    async fn upload_image(&self, blog_id: String, image_data: Vec<u8>) -> Result<String> {
        // Implement the logic to upload the image and return the URL
        unimplemented!()
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[sqlx::test]
    async fn test_create_draft_can_fetch_id(pool: sqlx::PgPool) -> Result<()> {
        let repo = Repository::new(pool);

        let draft_id = repo.create_draft().await;

        if let Ok(id) = draft_id {
            assert!(!id.is_empty());
        }

        Ok(())
    }
}