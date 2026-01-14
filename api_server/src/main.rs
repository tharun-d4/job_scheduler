use api_server::error;

#[tokio::main]
async fn main() -> Result<(), error::ServerError> {
    dotenvy::dotenv().ok();

    let app = api_server::build_app().await?;

    let host = std::env::var("SERVER_HOST").expect("SERVER_HOST env variable not found");
    let port = std::env::var("SERVER_PORT").expect("SERVER_PORT env variable not found");

    let bind = format!("{}:{}", host, port);
    println!("[+] Server running on {bind:?}...");

    let listener = tokio::net::TcpListener::bind(bind).await.unwrap();

    axum::serve(listener, app).await.unwrap();
    Ok(())
}
