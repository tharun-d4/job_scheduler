#[tokio::main]
async fn main() -> Result<(), worker::error::WorkerError> {
    worker::init().await
}
