use api_server::{app, db::connection, error, state};

#[tokio::main]
async fn main() -> Result<(), error::ServerError> {
    dotenvy::dotenv().ok();

    let pool = connection::create_pool().await?;
    connection::run_migrations(&pool).await?;

    let state = state::AppState::new(pool);
    let app = app::create_router(state);

    let host = std::env::var("SERVER_HOST").expect("SERVER_HOST env variable not found");
    let port = std::env::var("SERVER_PORT").expect("SERVER_PORT env variable not found");

    let bind = format!("{}:{}", host, port);
    println!("[+] Server running on {bind:?}...");

    let listener = tokio::net::TcpListener::bind(bind).await.unwrap();

    axum::serve(listener, app).await.unwrap();
    Ok(())
}
