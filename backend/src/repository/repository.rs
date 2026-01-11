use sqlx::PgPool;

pub struct Repository {
    pub pool: PgPool,
}

impl Repository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }
}