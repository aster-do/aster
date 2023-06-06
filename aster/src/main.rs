use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    time::Duration,
};

use axum::{routing::get, Router, Server};
use billing::services::BillingService;
use common::{services::AsterServiceError, AsterService};
use log::{error, info};
use tokio::join;

const LIVENESS_SERVER_ADDRESS: &SocketAddr =
    &SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 3029);
const LIVENESS_SERVER_ENDPOINT: &str = "/health";

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    //console subscriber for tokio-console debugger
    console_subscriber::ConsoleLayer::builder()
        .retention(Duration::from_secs(60))
        .server_addr(([127, 0, 0, 1], 5555))
        .init();

    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    info!("Starting Aster");
    let liveness_app = Router::new().route(LIVENESS_SERVER_ENDPOINT, get(liveness_handler));
    let liveness_server =
        Server::bind(LIVENESS_SERVER_ADDRESS).serve(liveness_app.into_make_service());

    let mut _messager = common::messaging::tokio_broadcast::CrossbeamMessagingFactory::default();

    // Create and init here
    info!("Initializing services");
    let mut services: Vec<Box<dyn AsterService>> = vec![
        Box::<frontend_server::FrontendServer>::default(),
        Box::<BillingService>::default(),
        Box::<controller::ControllerService>::default(),
        Box::<dashboard::DashboardServer>::default(),
        Box::<connector::ConnectorService>::default(),
        Box::<billable::BillableBuilderService>::default(),
        Box::<aggregator::BillableAggregatorService>::default(),
        Box::<alerting::AlertingInterface>::default(),
        Box::<notification::NotificationInterface>::default(),
        Box::<collector::ConnectorService>::default(),
    ];

    info!("Init services");
    for service in services.iter_mut() {
        service
            .init(&mut _messager)
            .await
            .map_err(AsterServiceError::AsterServiceInitFailed)?;
    }

    let mut handles = vec![];

    info!("Running services");
    for service in services.iter_mut() {
        handles.push(service.run());
    }

    info!("Running liveness server");
    let (services_result, liveness_result) =
        join!(futures::future::join_all(handles), liveness_server);

    // Handle post execution errors
    services_result.iter().for_each(|result| {
        if let Err(e) = result {
            error!("Service failed: {}", e);
        }
    });

    if let Err(e) = liveness_result {
        error!("Liveness server failed: {}", e);
    }

    Ok(())
}

async fn liveness_handler() -> &'static str {
    "OK"
}
