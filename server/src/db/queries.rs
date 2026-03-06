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

pub async fn mark_retry_exhausted_jobs_as_failed(pool: &PgPool) -> Result<u64, sqlx::Error> {
    let moved = query!(
        "
        UPDATE jobs
        SET status = 'failed',
            finished_at = NOW()
        WHERE status = 'pending'
            AND attempts >= max_retries
        ",
    )
    .execute(pool)
    .await?
    .rows_affected();

    Ok(moved)
}
