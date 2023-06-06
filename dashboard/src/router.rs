use std::sync::Arc;

use crate::routes::billables::get_billables;
use axum::{routing::get, Router};
use common::monitoring::readiness_handler;
use sqlx::PgPool;
use tower_http::cors::CorsLayer;

const READINESS_SERVER_ENDPOINT: &str = "/health";

const READINESS_SERVER_ENDPOINT: &str = "/health";

pub struct AppState {
    pub pool: PgPool,
}

pub fn get_router(pool: PgPool) -> Router {
    let app_state = Arc::new(AppState { pool });

    Router::new()
        .route("/billables", get(get_billables))
        .route(READINESS_SERVER_ENDPOINT, get(readiness_handler))
        .layer(CorsLayer::permissive())
        .with_state(app_state)
}
