use std::sync::Arc;

use axum::{
    Json,
    extract::{Path, State},
    http::StatusCode,
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value as JsonValue;
use tracing::{info, instrument};
use uuid::Uuid;

use crate::{error::ServerError, state::AppState};
use shared::db::{
    models::{Job, JobStatus, NewJob},
    queries::{get_job_by_id, insert_job},
};

#[derive(Debug, Deserialize)]
pub struct JobPayload {
    job_type: String,
    payload: JsonValue,
    priority: Option<i16>,
    max_retries: Option<i16>,
    schedule_at: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct JobId {
    pub job_id: Uuid,
}

#[instrument(skip(state))]
pub async fn create_job(
    State(state): State<Arc<AppState>>,
    Json(job_payload): Json<JobPayload>,
) -> Result<(StatusCode, Json<JobId>), ServerError> {
    let job_id = insert_job(
        &state.pool,
        NewJob {
            job_type: job_payload.job_type,
            payload: job_payload.payload,
            status: JobStatus::Pending,
            priority: job_payload.priority.unwrap_or(1),
            max_retries: job_payload.max_retries.unwrap_or(1),
            created_at: Utc::now(),
            run_at: job_payload.schedule_at.unwrap_or(Utc::now()),
        },
    )
    .await?;
    info!(%job_id, "Job Created");
    Ok((StatusCode::CREATED, Json(JobId { job_id })))
}

#[instrument(skip(state))]
pub async fn get_job(
    State(state): State<Arc<AppState>>,
    Path(id): Path<Uuid>,
) -> Result<Json<Job>, ServerError> {
    if let Some(job) = get_job_by_id(&state.pool, id).await {
        Ok(Json(job))
    } else {
        Err(ServerError::NotFound("Job Not Found".to_string()))
    }
}
