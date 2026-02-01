use super::super::model::blog::*;
use async_trait::async_trait;

#[async_trait]
pub trait BlogRepository {
    async fn get_blogs(&self, filter: BlogFilter) -> Vec<Blog>;
}