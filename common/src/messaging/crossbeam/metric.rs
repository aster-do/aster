use crate::models::Metric;

use super::{CrossbeamReceiver, CrossbeamSender};

pub type MetricSender = CrossbeamSender<Metric>;
pub type MetricReceiver = CrossbeamReceiver<Metric>;
