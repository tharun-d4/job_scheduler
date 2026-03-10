use sqlx::FromRow;

#[derive(Debug, PartialEq, FromRow, serde::Serialize)]
pub struct JobStats {
    pub pending: i64,
    pub running: i64,
    pub completed: i64,
    pub failed: i64,
}

#[derive(Debug, PartialEq, FromRow, serde::Serialize)]
pub struct JobStatsByJobType {
    pub job_type: String,
    pub pending: i64,
    pub running: i64,
    pub completed: i64,
    pub failed: i64,
}
