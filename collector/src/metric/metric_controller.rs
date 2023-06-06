use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use axum_macros::debug_handler;
use common::{messaging::AsyncSender, models::Metric};
use log::{debug, warn};

use super::model::AxumAppState;

// Handler to create a new metric
#[debug_handler]
pub async fn create_metric(
    State(state): State<AxumAppState>,
    Json(metric): Json<Metric>,
) -> impl IntoResponse {
    let mut metric_sender = state.metric_sender;
    let metric_clone = metric.clone();
    match metric_sender.send(metric_clone).await {
        Ok(_) => {
            // Metric sent successfully
            debug!("Metric controller sends following metric : {:?}", metric);

            StatusCode::CREATED
        }
        Err(err) => {
            // Error occurred while sending metric
            warn!(
                "Metric controller issued an error while sending the metric: {:?}, error: {}",
                metric, err
            );

            StatusCode::INTERNAL_SERVER_ERROR
        }
    }
}

#[cfg(test)]
mod tests {

    use chrono::{DateTime, Utc};

    use super::*;

    #[test]
    fn test_parse_metric() {
        let raw_metric_string = r#"{"corelation_id":"20","name":"cpu","timestamp": "2023-06-06T13:23:04+00:00", "value":20.0}"#;

        let metric_json = serde_json::from_str::<Metric>(raw_metric_string);

        assert!(metric_json.is_ok());

        let metric_unwrap = metric_json.unwrap();
        assert_eq!(metric_unwrap.corelation_id.unwrap(), "20".to_string());
        assert_eq!(metric_unwrap.name, "cpu".to_string());
        assert_eq!(metric_unwrap.value, 20.0);
        let expected_timestamp = DateTime::parse_from_rfc3339("2023-06-06T13:23:04+00:00")
            .unwrap()
            .with_timezone(&Utc)
            .to_rfc3339();

        assert_eq!(metric_unwrap.timestamp.to_rfc3339(), expected_timestamp);
    }
}
