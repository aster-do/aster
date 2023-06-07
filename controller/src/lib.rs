use axum::{
    extract::State,
    routing::{get, post},
    Json, Router, Server,
};
use axum_macros::debug_handler;
use billable_rule_service::BillableRuleService;
use common::{
    messaging::tokio_broadcast::CrossbeamMessagingFactory,
    models::billable_rules::{
        billable_rule_dto::BillableRuleDto, billable_rule_persistent::BillableRulePersistent,
    },
    monitoring::readiness_handler,
    AsterService,
};
use log::info;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

pub mod billable_rule_service;

const SERVER_ADDRESS: &SocketAddr = &SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3032);
const READINESS_SERVER_ENDPOINT: &str = "/health";

#[derive(Clone)]
pub struct AppState {
    pub billable_rules_service: BillableRuleService,
}

pub struct ControllerService {
    pub billable_rules_service: Option<BillableRuleService>,
}

impl ControllerService {
    pub fn new() -> Self {
        Self {
            billable_rules_service: None,
        }
    }
}

impl Default for ControllerService {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait::async_trait]
impl AsterService for ControllerService {
    async fn init(
        &mut self,
        _messaging: &mut CrossbeamMessagingFactory,
    ) -> Result<(), anyhow::Error> {
        info!("ControllerService is initializing");

        let database_url = std::env::var("DATABASE_URL")
            .unwrap_or("postgres://postgres:postgres@localhost:5432/postgres".to_string());

        self.billable_rules_service = Some(BillableRuleService::new(&database_url).await);

        Ok(())
    }

    async fn run(&mut self) -> Result<(), anyhow::Error> {
        info!("ControllerService is running");

        let billable_rule_service = self.billable_rules_service.as_ref().unwrap().clone();

        let state = AppState {
            billable_rules_service: billable_rule_service.clone(),
        };

        let readiness_app = Router::new()
            .route(READINESS_SERVER_ENDPOINT, get(readiness_handler))
            .route("/rules", get(get_billable_rules))
            .route("/rules", post(post_billable_rules))
            .with_state(state);

        Server::bind(SERVER_ADDRESS)
            .serve(readiness_app.into_make_service())
            .await?;

        Ok(())
    }
}

#[debug_handler]
async fn get_billable_rules(State(state): State<AppState>) -> Json<Vec<BillableRulePersistent>> {
    log::debug!("Received request for controller:");
    let billables: Vec<BillableRulePersistent> =
        state.billable_rules_service.get_all().await.unwrap();

    Json(billables)
}

#[debug_handler]
async fn post_billable_rules(
    State(state): State<AppState>,
    Json(dto): Json<BillableRuleDto>,
) -> Json<BillableRulePersistent> {
    log::debug!("Received request for controller:");

    let billable = state.billable_rules_service.create(&dto).await.unwrap();

    Json(billable)
}

#[cfg(test)]
mod tests {}
