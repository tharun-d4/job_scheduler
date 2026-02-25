use sqlx::postgres::PgPool;
use tracing::{error, info};
use uuid::Uuid;

use crate::db::queries::update_heartbeat;

pub async fn start_heartbeat_task(
    pool: PgPool,
    worker_id: Uuid,
    heartbeat: u8,
) -> tokio::task::JoinHandle<()> {
    tokio::spawn(async move {
        let mut interval = tokio::time::interval(std::time::Duration::from_secs(heartbeat as u64));
        loop {
            interval.tick().await;

            match update_heartbeat(&pool, worker_id).await {
                Ok(updated) => {
                    if updated == 1 {
                        info!(worker_id = %worker_id, "Heartbeat sent");
                    } else {
                        error!(worker_id = %worker_id, "Heartbeat update failed");
                    }
                }
                Err(e) => error!(worker_id = %worker_id, error = ?e, "Heartbeat failed"),
            }
        }
    })
}
