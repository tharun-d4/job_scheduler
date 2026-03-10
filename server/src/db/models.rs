use sqlx::FromRow;

#[derive(Debug, FromRow, serde::Serialize)]
pub struct JobStats {
    pending: i64,
    running: i64,
    completed: i64,
    failed: i64,
}

#[derive(Debug, FromRow, serde::Serialize)]
pub struct JobStatsByJobType {
    job_type: String,
    pending: i64,
    running: i64,
    completed: i64,
    failed: i64,
}
