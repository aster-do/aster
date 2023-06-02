use crate::models::Billable;

use super::{TokioBroadcastReceiver, TokioBroadcastSender};

pub type BillableSender = TokioBroadcastSender<Billable>;
pub type BillableReceiver = TokioBroadcastReceiver<Billable>;
