use std::sync::Arc;

use crate::routes::billables::get_billables;
use axum::{routing::get, Router};
use sqlx::PgPool;

pub struct AppState {
    pub pool: PgPool,
}

pub fn get_router(pool: PgPool) -> Router {
    let app_state = Arc::new(AppState { pool });

    Router::new()
        .route("/billables", get(get_billables))
        .with_state(app_state)
}
