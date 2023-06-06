use std::sync::Arc;

use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use common::models::Billable;

use crate::{dto::filters::BillableFilter, router::AppState};

#[allow(unused_variables)]
pub async fn get_billables(
    State(state): State<Arc<AppState>>,
    Query(query): Query<BillableFilter>,
) -> Result<Json<Vec<Billable>>, (StatusCode, String)> {
    log::debug!("Received request for billables: {:?}", query);
    let billables: Vec<Billable> = vec![];

    Ok(Json(billables))
}
