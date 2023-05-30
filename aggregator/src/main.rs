mod bills;

use tokio::time::{sleep, Duration};
use log::{ info, error, debug };
use bills::billable_controller::BillableController;

#[tokio::main]
async fn main(){
    env_logger::init();
    let url = std::env::var("PG_STRING").expect("PG_STRING must be set");

    info!("Starting aggregator");
    debug!("Connecting to postgres at {}", url);

    let mut controller = BillableController::new(&url)
        .await
        .expect("Failed to connect to postgres");

    controller.run_aggregation_pipeline().await.unwrap();
}

