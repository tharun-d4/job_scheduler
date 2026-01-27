use sqlx::postgres::PgPool;
use uuid::Uuid;

use worker::db::queries;

#[sqlx::test(migrations = "../migrations")]
async fn register_worker(pool: PgPool) {
    let worker_id = Uuid::now_v7();
    let pid = std::process::id();

    queries::register(&pool, worker_id, pid as i32)
        .await
        .unwrap();
}
