use reqwest::Client;
use sqlx::types::JsonValue;
use tracing::info;

use crate::error::WorkerError;

pub async fn send_webhook(
    client: Client,
    payload: JsonValue,
) -> Result<Option<JsonValue>, WorkerError> {
    let Some(url) = payload["url"].as_str() else {
        return Err(WorkerError::permanent("Invalid url"));
    };
    let method = payload["method"].as_str().unwrap_or("POST");
    let body = payload["body"].clone();

    let request = match method {
        "POST" => client.post(url),
        "PUT" => client.put(url),
        "PATCH" => client.patch(url),
        _ => return Err(WorkerError::permanent("Invalid method")),
    };

    let api_result = request
        .json(&body)
        .timeout(std::time::Duration::from_secs(30))
        .send()
        .await;

    match api_result {
        Ok(response) => {
            info!("response: {:?}", response);
            if response.status().is_success() {
                let response_json = response.json::<JsonValue>().await.map_err(|e| {
                    WorkerError::permanent("Failed to deserialize webhook response").set_source(e)
                })?;
                info!("response_json: {:?}", response_json);

                Ok(Some(response_json))
            } else {
                let status = response.status();
                if status == 429 || status.is_server_error() {
                    Err(WorkerError::temporary(&format!(
                        "Got a temporary failure status: {:?}",
                        status
                    )))
                } else {
                    Err(WorkerError::permanent(&format!(
                        "Got a permanent failure status: {:?}",
                        status
                    )))
                }
            }
        }
        Err(e) if e.is_connect() => Err(WorkerError::temporary("Connection error").set_source(e)),

        Err(e) if e.is_timeout() => {
            Err(WorkerError::temporary("Request got timed out").set_source(e))
        }
        Err(e) if e.is_decode() => {
            Err(WorkerError::permanent("Failed to deserialize webhook response").set_source(e))
        }

        Err(e) if e.is_redirect() => {
            Err(WorkerError::permanent("Redirected so misconfigured url").set_source(e))
        }

        Err(e) => Err(WorkerError::temporary("Unknown error").set_source(e)),
    }
}
