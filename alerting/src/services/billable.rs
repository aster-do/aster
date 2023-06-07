use anyhow::{Ok, Result};
use common::models::Billable;
use notification::services::notification::Notificationservice;

#[derive(Debug)]
pub struct BillableService {
    //Config & stateful info
    _notification_service: Notificationservice,
}

impl BillableService {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            //Config & stateful info
            _notification_service: Notificationservice::new()?,
        })
    }

    pub async fn _handle_billable(&self, _billable: Billable) -> Result<()> {
        //TODO: Implement check if billable trigger alert logic
        Ok(())
    }
}
