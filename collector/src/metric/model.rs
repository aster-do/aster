use common::messaging::tokio_broadcast::MetricSender;

// State stored by axum to send metrics
#[derive(Clone)]
pub struct AxumAppState {
    pub metric_sender: MetricSender,
}

impl AxumAppState {
    pub fn new(metric_sender: MetricSender) -> Self {
        AxumAppState { metric_sender }
    }
}
