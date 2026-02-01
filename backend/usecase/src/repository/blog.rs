use super::super::model::blog::*;
use async_trait::async_trait;

#[async_trait]
pub trait BlogRepository: Send + Sync {
    async fn get_blogs(&self, filter: BlogFilter) -> Vec<Blog>;
}