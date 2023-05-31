use axum::{
    body::Body,
    extract::State,
    http::{Response, StatusCode},
    Json,
};
use axum_macros::debug_handler;
use common::{messaging::AsyncSender, models::Metric};
use log::{debug, warn};

use super::model::AxumAppState;

// Handler to create a new metric
#[debug_handler]
pub async fn create_metric(
    State(state): State<AxumAppState>,
    Json(metric): Json<Metric>,
) -> Response<Body> {
    let mut metric_sender = state.metric_sender;
    let metric_json = serde_json::to_string(&metric).unwrap();

    match metric_sender.send(metric).await {
        Ok(_) => {
            // Metric sent successfully
            debug!(
                "Metric controller sends following metric : {:?}",
                metric_json
            );

            Response::builder()
                .status(StatusCode::CREATED)
                .body(Body::from(metric_json))
                .unwrap()
        }
        Err(err) => {
            // Error occurred while sending metric
            warn!(
                "Metric controller issued an error while sending the metric: {}, error: {}",
                metric_json, err
            );

            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(Body::from(format!(
                    "Failed to send metric: {}, error: {}",
                    err, metric_json
                )))
                .unwrap()
        }
    }
}
