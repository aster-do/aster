use crate::models::Metric;

use super::{TokioBroadcastReceiver, TokioBroadcastSender};

pub type MetricSender = TokioBroadcastSender<Metric>;
pub type MetricReceiver = TokioBroadcastReceiver<Metric>;
