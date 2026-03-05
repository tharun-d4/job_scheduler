use std::fmt;

#[derive(Debug, PartialEq)]
pub enum ErrorStatus {
    Temporary,
    Permanent,
}

#[derive(Debug)]
pub struct WorkerError {
    pub status: ErrorStatus,
    pub message: String,
    pub source: Option<anyhow::Error>,
}

impl fmt::Display for WorkerError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self:?}")
    }
}

impl core::error::Error for WorkerError {}

impl WorkerError {
    pub fn permanent(message: &str) -> Self {
        Self {
            status: ErrorStatus::Permanent,
            message: message.to_string(),
            source: None,
        }
    }
    pub fn temporary(message: &str) -> Self {
        Self {
            status: ErrorStatus::Temporary,
            message: message.to_string(),
            source: None,
        }
    }
    pub fn set_source(mut self, source: impl Into<anyhow::Error>) -> Self {
        self.source = Some(source.into());
        self
    }

    pub fn is_permanent(&self) -> bool {
        self.status == ErrorStatus::Permanent
    }
}
