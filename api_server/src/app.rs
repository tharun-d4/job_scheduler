use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};

use crate::{handlers::create_task, state::AppState};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(|| async { "Hello World" }))
        .route("/tasks", post(create_task))
        .with_state(Arc::new(state))
}
