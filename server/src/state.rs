use sqlx::postgres::PgPool;

pub struct AppState {
    pub pool: PgPool,
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        AppState { pool }
    }
}
