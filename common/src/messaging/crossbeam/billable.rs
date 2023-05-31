use crate::models::Billable;

use super::{CrossbeamReceiver, CrossbeamSender};

pub type BillableSender = CrossbeamSender<Billable>;
pub type BillableReceiver = CrossbeamReceiver<Billable>;
