use crate::repository::repository::Repository;

use super::super::model::blog::*;

pub trait BlogRepository {
    async fn get_blogs(&self, filter: BlogFilter);
}

impl BlogRepository for Repository {
    async fn get_blogs(&self, filter: BlogFilter) {

        // let rows = self.client.query("SELECT * FROM blogs", &[]).await.unwrap();
        return;
    }
}