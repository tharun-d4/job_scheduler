use std::sync::Arc;

use axum::{
    Router,
    routing::{get, post},
};

use crate::{handlers::create_job, state::AppState};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/", get(|| async { "Hello World" }))
        .route("/jobs", post(create_job))
        .with_state(Arc::new(state))
}
