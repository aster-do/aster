use crate::models::Metric;

use super::{CrossbeamReceiver, CrossbeamSender};

pub type MetricSender = crossbeam_channel::Sender<Metric>;
pub type MetricReceiver = crossbeam_channel::Receiver<Metric>;
