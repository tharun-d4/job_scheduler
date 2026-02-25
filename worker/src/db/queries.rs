use chrono::{TimeDelta, Utc};
use sqlx::{postgres::PgPool, query, query_as, types::JsonValue};
use uuid::Uuid;

use shared::db::models::{Job, JobStatus};

pub async fn register(pool: &PgPool, worker_id: Uuid, pid: i32) -> Result<(), sqlx::Error> {
    query("INSERT INTO workers (id, pid, started_at, last_heartbeat) VALUES ($1, $2, $3, $3);")
        .bind(worker_id)
        .bind(pid)
        .bind(Utc::now())
        .execute(pool)
        .await?;
    Ok(())
}

pub async fn update_heartbeat(pool: &PgPool, worker_id: Uuid) -> Result<(), sqlx::Error> {
    query("UPDATE workers SET last_heartbeat=$2 WHERE id=$1;")
        .bind(worker_id)
        .bind(Utc::now())
        .execute(pool)
        .await?;

    Ok(())
}

pub async fn claim_job(
    pool: &PgPool,
    worker_id: Uuid,
    lease_duration: u8,
) -> Result<Job, sqlx::Error> {
    query_as::<_, Job>(
        "UPDATE jobs
        SET
            status = $1,
            worker_id = $2,
            started_at = $3,
            lease_expires_at = $4,
            attempts = attempts + 1
        WHERE id = (
            SELECT id FROM jobs
            WHERE status = $5
            AND attempts < max_retries
            AND run_at < NOW()
            ORDER BY priority DESC, created_at ASC
            LIMIT 1
        )
        RETURNING *",
    )
    .bind(JobStatus::Running)
    .bind(worker_id)
    .bind(Utc::now())
    .bind(Utc::now() + TimeDelta::seconds(lease_duration as i64))
    .bind(JobStatus::Pending)
    .fetch_one(pool)
    .await
}

pub async fn mark_job_as_completed(
    pool: &PgPool,
    job_id: Uuid,
    worker_id: Uuid,
    result: Option<JsonValue>,
) -> Result<(), sqlx::Error> {
    query(
        "UPDATE jobs
        SET
            status = $1,
            completed_at = $2,
            result = $3
        WHERE id = $4
        AND worker_id = $5
        AND status = $6;",
    )
    .bind(JobStatus::Completed)
    .bind(Utc::now())
    .bind(result)
    .bind(job_id)
    .bind(worker_id)
    .bind(JobStatus::Running)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn store_job_error(
    pool: &PgPool,
    job_id: Uuid,
    worker_id: Uuid,
    error: String,
    backoff_secs: i16,
) -> Result<(), sqlx::Error> {
    query(
        "UPDATE jobs
        SET
            status = $1,
            error_message = $2,
            run_at = NOW() + ($3 * INTERVAL '1 SECONDS')
        WHERE id = $4
        AND worker_id = $5;",
    )
    .bind(JobStatus::Pending)
    .bind(error)
    .bind(backoff_secs)
    .bind(job_id)
    .bind(worker_id)
    .execute(pool)
    .await?;

    Ok(())
}

pub async fn update_worker_shutdown_time(
    pool: &PgPool,
    worker_id: Uuid,
) -> Result<(), sqlx::Error> {
    query(
        "UPDATE workers
        SET shutdown_at = NOW()
        WHERE id = $1;",
    )
    .bind(worker_id)
    .execute(pool)
    .await?;

    Ok(())
}
