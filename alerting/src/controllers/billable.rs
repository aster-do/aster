use anyhow::{Ok, Result};
use common::messaging::crossbeam::BillableReceiver;
use log::{debug, info};

use common::messaging::AsyncReceiver;

use crate::services::billable::BillableService;

pub struct BillableController {
    //Config & stateful info
    _billable_service: BillableService,
    billable_receiver: BillableReceiver,
}

impl BillableController {
    pub fn new(billable_receiver: BillableReceiver) -> Result<Self> {
        Ok(Self {
            //Config & stateful info
            _billable_service: BillableService::new()?,
            billable_receiver,
        })
    }

    pub async fn start(&self) -> Result<()> {
        info!("Listening for billable messages");

        let mut receiver = self.billable_receiver.clone();

        loop {
            let billable = receiver.receive().await?;
            debug!("Received billable message: {:?}", billable);
            //TODO: Process billable
        }
    }
}
