use chrono::Utc;
use sqlx::{PgPool, types::JsonValue};
use uuid::Uuid;

use shared::db::{
    models::{JobStatus, NewJob},
    queries,
};

#[sqlx::test(migrations = "../migrations")]
async fn test_insert_job_returns_job_id(pool: PgPool) -> Result<(), sqlx::Error> {
    queries::insert_job(
        &pool,
        NewJob {
            job_type: "new_job".to_string(),
            payload: JsonValue::String("A new job".to_string()),
            status: JobStatus::Pending,
            priority: 1,
            max_retries: 5,
            created_at: Utc::now(),
        },
    )
    .await?;
    Ok(())
}

#[sqlx::test(
    migrations = "../migrations",
    fixtures(path = "../../test_fixtures", scripts("jobs"))
)]
async fn test_get_job_by_id(pool: PgPool) -> Result<(), sqlx::Error> {
    let job_id = "019bfadc-28bb-781d-9d22-acf23fe50117"
        .parse::<Uuid>()
        .unwrap();
    let job = queries::get_job_by_id(&pool, job_id).await;

    assert!(job.is_some());
    assert_eq!(job_id, job.unwrap().id);
    Ok(())
}
