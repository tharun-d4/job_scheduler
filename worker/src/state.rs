use std::sync::Arc;

use lettre::{AsyncSmtpTransport, Tokio1Executor};
use prometheus_client::registry::Registry;
use reqwest::Client;

use crate::prometheus::Metrics;

pub struct AppState {
    pub registry: Arc<Registry>,
    pub metrics: Arc<Metrics>,
    pub client: Client,
    pub smtp_sender: AsyncSmtpTransport<Tokio1Executor>,
}
