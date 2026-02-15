use sqlx::PgPool;
use usecase::repository::repositories::Reporisories;

pub struct Repository {
    pub pool: PgPool,
}

impl Repository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}

impl Reporisories for Repository {
}