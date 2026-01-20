use axum_test::TestServer;
use sqlx::PgPool;

use server::{app, state};

pub fn build_test_server(pool: PgPool) -> TestServer {
    let state = state::AppState::new(pool);
    let app = app::create_router(state);

    TestServer::new(app).unwrap()
}
