use prometheus_client::{
    encoding::EncodeLabelSet,
    metrics::{counter::Counter, family::Family, histogram::Histogram},
    registry::Registry,
};

#[derive(Debug, Clone, PartialEq, Eq, Hash, EncodeLabelSet)]
pub struct JobType {
    pub job_type: String,
}

pub struct Metrics {
    pub jobs_completed: Family<JobType, Counter>,
    pub jobs_failed: Family<JobType, Counter>,
    pub jobs_retried: Family<JobType, Counter>,
    pub jobs_processing_duration_seconds: Family<JobType, Histogram>,
}

pub fn register_metrics() -> (Registry, Metrics) {
    let mut registry = Registry::default();

    let metrics = Metrics {
        jobs_completed: Family::default(),
        jobs_failed: Family::default(),
        jobs_retried: Family::default(),
        jobs_processing_duration_seconds: Family::new_with_constructor(|| {
            Histogram::new([
                0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0,
            ])
        }),
    };

    registry.register(
        "jobs_completed",
        "Total jobs completed",
        metrics.jobs_completed.clone(),
    );
    registry.register(
        "jobs_failed",
        "Total jobs failed",
        metrics.jobs_failed.clone(),
    );
    registry.register(
        "jobs_retried",
        "Total jobs retried",
        metrics.jobs_retried.clone(),
    );
    registry.register(
        "jobs_processing_duration_seconds",
        "Job processing duration in seconds",
        metrics.jobs_processing_duration_seconds.clone(),
    );

    (registry, metrics)
}
