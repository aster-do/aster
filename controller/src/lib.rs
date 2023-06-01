use billable_rule_service::BillableRuleService;
use common::{messaging::crossbeam::CrossbeamMessagingFactory, AsterService};
use log::info;

pub mod billable_rule_service;

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

        let rules = self
            .billable_rules_service
            .as_ref()
            .unwrap()
            .get_all()
            .await?;

        info!("{:?}", rules);

        Ok(())
    }
}

#[cfg(test)]
mod tests {}
