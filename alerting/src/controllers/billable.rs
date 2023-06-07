use common::messaging::tokio_broadcast::BillableReceiver;
use log::{debug, info};

use common::messaging::AsyncReceiver;

use crate::services::billable::BillableService;

pub struct BillableController {
    //Config & stateful info
    billable_service: BillableService,
    billable_receiver: BillableReceiver,
}

impl BillableController {
    pub async fn new(billable_receiver: BillableReceiver) -> Result<Self, anyhow::Error> {
        Ok(Self {
            //Config & stateful info
            billable_service: BillableService::new().await?,
            billable_receiver,
        })
    }

    pub async fn start(&mut self) -> Result<(), anyhow::Error> {
        info!("Listening for billable messages");

        let mut receiver = self.billable_receiver.clone();

        loop {
            let billable = receiver.receive().await?;
            debug!("Received billable message: {:?}", billable);
            self.billable_service.handle_billable(billable).await?;
        }
    }
}
