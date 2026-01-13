use chrono::{DateTime, Utc};
use sqlx::types::JsonValue;
use uuid::Uuid;

#[derive(Debug, sqlx::Type)]
#[sqlx(type_name = "task_status", rename_all = "lowercase")]
pub enum TaskStatus {
    Pending,
    Running,
    Completed,
    Failed,
}

#[derive(Debug)]
pub struct Task {
    pub id: Uuid,
    pub task_type: String,
    pub payload: JsonValue,
    pub status: TaskStatus,
    pub priority: i8,
    pub max_retries: u8,
    pub created_at: DateTime<Utc>,
    pub started_at: Option<DateTime<Utc>>,
    pub completed_at: Option<DateTime<Utc>>,
    pub worker_id: Option<Uuid>,
    pub attempts: Option<u8>,
    pub error_message: Option<String>,
    pub result: Option<JsonValue>,
}

#[derive(Debug)]
pub struct NewTask {
    pub task_type: String,
    pub payload: JsonValue,
    pub status: TaskStatus,
    pub priority: i8,
    pub max_retries: u8,
    pub created_at: DateTime<Utc>,
}
