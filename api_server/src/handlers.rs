use std::sync::Arc;

use axum::{Json, extract::State, response::IntoResponse};
use chrono::Utc;
use serde::Deserialize;
use serde_json::Value as JsonValue;

use crate::{
    db::{models, queries},
    state::AppState,
};

#[derive(Debug, Deserialize)]
pub struct TaskPayload {
    task_type: String,
    payload: JsonValue,
    priority: Option<i8>,
    max_retries: Option<u8>,
}

pub async fn create_task(
    State(state): State<Arc<AppState>>,
    Json(req_payload): Json<TaskPayload>,
) -> impl IntoResponse {
    let state = state.clone();

    queries::insert_task(
        &state.pool,
        models::NewTask {
            task_type: req_payload.task_type,
            payload: req_payload.payload,
            status: models::TaskStatus::Pending,
            priority: match req_payload.priority {
                Some(val) => val,
                None => 1,
            },
            max_retries: match req_payload.max_retries {
                Some(val) => val,
                None => 1,
            },
            created_at: Utc::now(),
        },
    )
    .await
    .unwrap();
    "Create task called"
}
