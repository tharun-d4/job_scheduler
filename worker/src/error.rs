use lettre::{address::AddressError, transport::smtp::Error as SmtpError};
use sqlx::Error as SqlxError;

#[derive(Debug)]
pub enum WorkerError {
    Database(SqlxError),
    ParseEmail(AddressError),
    SmtpError(SmtpError),
}

impl From<SqlxError> for WorkerError {
    fn from(err: SqlxError) -> Self {
        WorkerError::Database(err)
    }
}

impl From<AddressError> for WorkerError {
    fn from(err: AddressError) -> Self {
        WorkerError::ParseEmail(err)
    }
}

impl From<SmtpError> for WorkerError {
    fn from(err: SmtpError) -> Self {
        WorkerError::SmtpError(err)
    }
}
