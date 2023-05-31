mod bills;

use tokio::time::{sleep, Duration};
use log::{ info, error, debug };
use bills::billable_controller::BillableController;

async fn run(){
    env_logger::init();
    let url = std::env::var("PG_STRING").expect("PG_STRING must be set");

    info!("Starting aggregator");
    debug!("Connecting to postgres at {}", url);

    let mut controller = BillableController::new(&url)
        .await
        .expect("Failed to connect to postgres");


    let mut fail_count = 0;
    const MAX_FAIL_COUNT: u32 = 5;

    while fail_count < MAX_FAIL_COUNT {
        match controller.run_aggregation_pipeline().await {
            Ok(_) => {
                info!("Aggregation pipeline completed successfully");
                fail_count = 0;
                sleep(Duration::from_secs(60)).await;
            },
            Err(e) => {
                error!("Aggregation pipeline failed: {}, retrying in 5 seconds...", e);
                fail_count += 1;
                sleep(Duration::from_secs(5)).await;
            }
        };

    }

    error!("Aggregation pipeline failed {} times in a row. Shutting down", MAX_FAIL_COUNT);
}