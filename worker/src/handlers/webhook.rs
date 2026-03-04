use reqwest::Client;
use sqlx::types::JsonValue;
use tracing::info;

use crate::error::WorkerErrorV2;

pub async fn send_webhook(
    client: Client,
    payload: JsonValue,
) -> Result<Option<JsonValue>, WorkerErrorV2> {
    let Some(url) = payload["url"].as_str() else {
        return Err(WorkerErrorV2::permanent("Invalid url"));
    };
    let method = payload["method"].as_str().unwrap_or("POST");
    let body = payload["body"].clone();

    let request = match method {
        "POST" => client.post(url),
        "PUT" => client.put(url),
        "PATCH" => client.patch(url),
        _ => return Err(WorkerErrorV2::permanent("Invalid method")),
    };

    let response = request
        .json(&body)
        .timeout(std::time::Duration::from_secs(30))
        .send()
        .await
        .map_err(|e| WorkerErrorV2::permanent("Failed to send webhook request").set_source(e))?;

    info!("response: {:?}", response);

    let response_json = response.json::<JsonValue>().await.map_err(|e| {
        WorkerErrorV2::permanent("Failed to deserialize webhook response").set_source(e)
    })?;

    info!("response_json: {:?}", response_json);

    Ok(Some(response_json))
}
