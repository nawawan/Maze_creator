use usecase::repository::blog::{BlogRepository};
use usecase::model::blog::{Blog, BlogFilter};
use super::repository::*;

use async_trait::async_trait;

#[async_trait]
impl BlogRepository for Repository {
    async fn get_blogs(&self, filter: BlogFilter) -> Vec<Blog> {

        // let rows = self.client.query("SELECT * FROM blogs", &[]).await.unwrap();
        return vec![];
    }
}