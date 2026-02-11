use sqlx::{postgres::PgPool, query};

pub async fn recover_unfinished_lease_expired_jobs(pool: &PgPool) -> Result<u64, sqlx::Error> {
    let jobs_recovered = query!(
        "
        UPDATE jobs
        SET status = 'pending',
        error_message = 'lease expired or worker crashed'
        WHERE status = 'running'
        AND lease_expires_at < NOW()
        ",
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(jobs_recovered)
}

pub async fn move_retry_exhausted_jobs_to_failed_jobs(pool: &PgPool) -> Result<u64, sqlx::Error> {
    let moved = query!(
        "
        WITH deleted_jobs AS (
            DELETE FROM jobs
            WHERE status = 'pending'
            AND attempts >= max_retries
            RETURNING
                id,
                job_type,
                payload,
                priority,
                max_retries,
                created_at,
                started_at,
                NOW(),
                worker_id,
                attempts,
                error_message,
                result,
                lease_expires_at
        )
        INSERT INTO failed_jobs
        SELECT * FROM deleted_jobs;
        ",
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(moved)
}
