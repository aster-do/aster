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

#[debug_handler]
pub async fn put_billable_rule(
    State(state): State<AppState>,
    Path(rule_id): Path<i32>,
    Json(dto): Json<BillableRuleDto>,
) -> Json<Option<BillableRulePersistent>> {
    log::debug!("Received request for controller:");

    let fetched_billable_rule = state.billable_rules_service.get_by_id(rule_id).await;

    let mut billable_rule = match fetched_billable_rule {
        Ok(rule) => match rule {
            Some(rule) => rule,
            None => {
                log::error!("Could not find billable rule with id: {}", rule_id);
                return Json(None);
            }
        },
        Err(e) => {
            log::error!("Error {} getting billable rule with id: {}", e, rule_id);
            return Json(None);
        }
    };

    billable_rule.id = rule_id;

    if billable_rule.update_from(&dto, rule_id).is_err() {
        log::error!("Error updating billable rule with id: {}", rule_id);
        return Json(None);
    }

    let persisted_billable_rule = state.billable_rules_service.update(&billable_rule).await;

    match persisted_billable_rule {
        Ok(rule) => Json(Some(rule)),
        Err(e) => {
            log::error!("Error updating billable rule with id: {}", e);
            Json(None)
        }
    }
}
