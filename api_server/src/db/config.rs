use sqlx::postgres::{PgPool, PgPoolOptions};

pub async fn create_app_db_pool() -> Result<PgPool, sqlx::Error> {
    let db_url = std::env::var("DATABASE_URL").unwrap();
    PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
}
