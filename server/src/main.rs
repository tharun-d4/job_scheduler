use tracing::{info, instrument};

use server::{app, cleanup, error, lease_recovery, state};
use shared::{config::load_server_config, db::connection, tracing::init_tracing};

#[instrument]
#[tokio::main]
async fn main() -> Result<(), error::ServerError> {
    let _trace_guard = init_tracing("server");
    let config = load_server_config("./config").expect("Config Error");

    let pool = connection::create_pool(&config.database).await?;
    connection::run_migrations(&pool).await?;

    lease_recovery::lease_recovery_task(pool.clone(), config.server.lease_recovery).await;
    cleanup::cleanup_task(pool.clone(), config.server.cleanup).await;

    let state = state::AppState::new(pool);
    let app = app::create_router(state);

    let bind = format!("{}:{}", config.server.host, config.server.port);
    info!("[+] Server running on {bind:?}...");

    let listener = tokio::net::TcpListener::bind(bind).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
