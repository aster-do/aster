use crate::models::Metric;


pub type MetricSender = crossbeam_channel::Sender<Metric>;
pub type MetricReceiver = crossbeam_channel::Receiver<Metric>;
