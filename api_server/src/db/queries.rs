use sqlx::{postgres::PgPool, query_scalar};
use uuid::Uuid;

use super::models::NewTask;

pub async fn insert_task(pool: &PgPool, task: NewTask) -> Result<Uuid, sqlx::Error> {
    let task_id = query_scalar(
        "INSERT INTO tasks
        VALUES (
            $1, $2, $3, $4, $5, $6, $7
        )
        RETURNING id",
    )
    .bind(task.id)
    .bind(task.task_type)
    .bind(task.payload)
    .bind(task.status)
    .bind(task.priority as i16)
    .bind(task.max_retries as i16)
    .bind(task.created_at)
    .fetch_one(pool)
    .await?;

    Ok(task_id)
}
