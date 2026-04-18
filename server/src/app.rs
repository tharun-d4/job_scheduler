use std::{sync::Arc, time::Instant};

use axum::{
    Router,
    extract::{Request, State},
    http::Method,
    middleware::{self, Next},
    response::Response,
    routing::get,
};
use tower::ServiceBuilder;
use tower_http::trace::{
    DefaultMakeSpan, DefaultOnFailure, DefaultOnRequest, DefaultOnResponse, TraceLayer,
};
use tracing::Level;

use crate::{
    handlers,
    prometheus::{HttpLabel, HttpMethod},
    state::AppState,
};

pub fn create_router(state: Arc<AppState>) -> Router {
    let trace_layer = ServiceBuilder::new().layer(
        TraceLayer::new_for_http()
            .make_span_with(DefaultMakeSpan::new().level(Level::INFO))
            .on_request(DefaultOnRequest::new().level(Level::INFO))
            .on_response(DefaultOnResponse::new().level(Level::INFO))
            .on_failure(DefaultOnFailure::new().level(Level::INFO)),
    );

    Router::new()
        .fallback(handlers::handler_404)
        .route("/metrics", get(handlers::get_metrics))
        .route("/jobs", get(handlers::list_jobs).post(handlers::create_job))
        .route(
            "/jobs/{id}",
            get(handlers::get_job_by_id).delete(handlers::cancel_job),
        )
        .route("/jobs/stats", get(handlers::job_stats))
        .route("/jobs/stats/detailed", get(handlers::detailed_job_stats))
        .layer(middleware::from_fn_with_state(state.clone(), track_metrics))
        .with_state(state)
        .layer(trace_layer)
}

async fn track_metrics(
    State(state): State<Arc<AppState>>,
    request: Request,
    next: Next,
) -> Response {
    let start = Instant::now();

    let method = match *request.method() {
        Method::GET => HttpMethod::GET,
        Method::POST => HttpMethod::POST,
        Method::PUT => HttpMethod::PUT,
        Method::DELETE => HttpMethod::DELETE,
        _ => unimplemented!("No API found with that HTTP method"),
    };
    let path = request.uri().path().to_owned();

    let response = next.run(request).await;

    let response_time = start.elapsed().as_secs_f64();

    let label = HttpLabel { method, path };

    state.metrics.http_requests.get_or_create(&label).inc();
    state
        .metrics
        .http_request_duration_seconds
        .get_or_create(&label)
        .observe(response_time);

    response
}
