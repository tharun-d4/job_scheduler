pub mod app;
pub mod db;
pub mod error;
pub mod handlers;
pub mod state;

pub async fn build_app() -> Result<axum::Router, error::ServerError> {
    let pool = db::connection::create_pool().await?;
    db::connection::run_migrations(&pool).await?;

    let state = state::AppState::new(pool);
    let app = app::create_router(state);

    Ok(app)
}
