use sqlx::Error as SqlxError;

#[derive(Debug)]
pub enum WorkerError {
    Database(SqlxError),
}

impl From<SqlxError> for WorkerError {
    fn from(err: SqlxError) -> Self {
        WorkerError::Database(err)
    }
}
