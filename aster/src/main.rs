use billing::services::BillingService;
use common::{services::AsterServiceError, AsterService};
use log::info;

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    info!("Starting Aster");

    let mut _messager = common::messaging::crossbeam::CrossbeamMessagingFactory::default();

    // Create and init here
    info!("Initializing services");
    let mut services: Vec<Box<dyn AsterService>> = vec![
        Box::<frontend_server::FrontendServer>::default(),
        Box::<BillingService>::default(),
        Box::<dashboard::DashboardServer>::default(),
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

    futures::future::join_all(handles).await;

    Ok(())
}
