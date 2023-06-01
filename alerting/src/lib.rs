use anyhow::Result;
use async_trait::async_trait;
use common::{messaging::MessagingFactory, AsterService};
use controllers::{billable::BillableController, rule::RuleController};
use log::debug;

mod controllers;
mod models;
mod services;

pub struct AlertingInterface {
    //Config & stateful info
    billable_controller: Option<BillableController>,
    rule_controller: Option<RuleController>,
}

//TODO Implement AsterService trait
#[async_trait]
impl AsterService for AlertingInterface {
    async fn init(
        &mut self,
        messaging: &mut common::messaging::crossbeam::CrossbeamMessagingFactory,
    ) -> Result<(), anyhow::Error> {
        debug!("Initializing billable controller");
        self.billable_controller = Some(BillableController::new(
            messaging.create_billable_receiver().await,
        )?);

        debug!("Initializing rule controller");
        self.rule_controller = Some(RuleController::new()?);

        Ok(())
    }

    async fn run(&mut self) -> Result<(), anyhow::Error> {
        //TODO Start servers
        debug!("Starting servers");
        Ok(())
    }
}
