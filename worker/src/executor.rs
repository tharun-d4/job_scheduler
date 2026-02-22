use lettre::{Tokio1Executor, transport::smtp::AsyncSmtpTransport};
use sqlx::{postgres::PgPool, types::JsonValue};
use tracing::{error, info, instrument};

use shared::db::models::Job;

use crate::{
    db::queries,
    error::WorkerError,
    handlers::{email, models::EmailInfo, webhook::send_webhook},
};

#[instrument(skip(pool, smtp_sender))]
pub async fn execute_job(
    pool: &PgPool,
    job: Job,
    smtp_sender: AsyncSmtpTransport<Tokio1Executor>,
    client: reqwest::Client,
) -> Result<(), WorkerError> {
    let job_id = job.id;

    let result = match job.job_type.as_ref() {
        "send_email" => send_email(smtp_sender, job).await,
        "send_webhook" => send_webhook(client, job.payload).await,
        "will_crash" => panic!("Worker will crash when running this job!"),
        _ => Err(WorkerError::InvalidJob),
    };

    match result {
        Ok(res) => {
            info!("Marking job as completed");
            queries::mark_job_as_completed(pool, job_id, res).await?;
        }
        Err(err) => {
            error!("Got error: {:?}", err);
            queries::store_job_error(pool, job_id, err.to_string()).await?;
        }
    }

    Ok(())
}

async fn send_email(
    smtp_sender: AsyncSmtpTransport<Tokio1Executor>,
    job: Job,
) -> Result<Option<JsonValue>, WorkerError> {
    let email_info: EmailInfo = serde_json::from_value(job.payload)
        .map_err(|e| WorkerError::Email(format!("email payload json error: {:?}", e)))?;

    info!("Sending an email: {:?}", email_info);
    email::send_email(smtp_sender, email_info).await?;
    Ok(None)
}

fn retry_backoff_secs(attempts: i16) -> i16 {
    2_i16.pow(attempts as u32)
}

#[cfg(test)]
mod tests {
    use super::retry_backoff_secs;

    #[test]
    fn backoff_secs_with_4_attempts_returns_16() {
        assert_eq!(retry_backoff_secs(4), 16);
    }

    #[test]
    fn backoff_secs_with_10_attempts_returns_1024() {
        assert_eq!(retry_backoff_secs(10), 1024);
    }
}
