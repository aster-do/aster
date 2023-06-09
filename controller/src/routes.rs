use super::billable_rule_service::BillableRuleService;
use axum::extract::{Path, State};
use axum::Json;
use axum_macros::debug_handler;
use common::models::billable_rules::{
    billable_rule_dto::BillableRuleDto, billable_rule_persistent::BillableRulePersistent,
};

#[derive(Clone)]
pub struct AppState {
    pub billable_rules_service: BillableRuleService,
}

#[debug_handler]
pub async fn get_billable_rules(
    State(state): State<AppState>,
) -> Json<Vec<BillableRulePersistent>> {
    log::debug!("Received request for controller:");
    let billables: Vec<BillableRulePersistent> =
        state.billable_rules_service.get_all().await.unwrap();

    Json(billables)
}

#[debug_handler]
pub async fn delete_billable_rule_by_id(
    State(state): State<AppState>,
    Path(rule_id): Path<i32>,
) -> Json<Option<BillableRulePersistent>> {
    log::debug!("Received request for controller:");

    let billable = state.billable_rules_service.delete(rule_id).await;

    match billable {
        Ok(_) => Json(None),
        Err(e) => {
            log::error!("Error: {}", e);
            Json(None)
        }
    }
}

#[debug_handler]
pub async fn get_billable_rule_by_id(
    State(state): State<AppState>,
    Path(rule_id): Path<i32>,
) -> Json<Option<BillableRulePersistent>> {
    log::debug!("Received request for controller:");

    let billable = state.billable_rules_service.get_by_id(rule_id).await;

    match billable {
        Ok(billable) => Json(billable),
        Err(e) => {
            log::error!("Error: {}", e);
            Json(None)
        }
    }
}

#[debug_handler]
pub async fn post_billable_rules(
    State(state): State<AppState>,
    Json(dto): Json<BillableRuleDto>,
) -> Json<BillableRulePersistent> {
    log::debug!("Received request for controller:");

    let billable = state.billable_rules_service.create(&dto).await.unwrap();

    Json(billable)
}
