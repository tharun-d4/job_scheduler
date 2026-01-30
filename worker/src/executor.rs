use tracing::info;

use shared::db::models::Job;

pub async fn execute_job(job: Job) {
    match job.job_type.as_ref() {
        "send_email" => {
            info!("Got a send_email job to do. Performing it ...");
        }
        _ => {}
    }
}
