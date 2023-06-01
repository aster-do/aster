use crate::models::Billable;

// use super::{CrossbeamReceiver, CrossbeamSender};

pub type BillableSender = crossbeam_channel::Sender<Billable>;
pub type BillableReceiver = crossbeam_channel::Receiver<Billable>;
