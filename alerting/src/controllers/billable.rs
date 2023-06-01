use anyhow::{Ok, Result};
use common::messaging::crossbeam::BillableReceiver;

use crate::services::billable::BillableService;

pub struct BillableController {
    //Config & stateful info
    _billable_service: BillableService,
    _billable_receiver: BillableReceiver,
}

impl BillableController {
    pub fn new(_billable_receiver: BillableReceiver) -> Result<Self> {
        Ok(Self {
            //Config & stateful info
            _billable_service: BillableService::new()?,
            _billable_receiver,
        })
    }

    pub fn _start(&self) -> Result<()> {
        //TODO Start billable receiver
        Ok(())
    }
}

//TODO Implement topic communication trait
