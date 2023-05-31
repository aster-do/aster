use common::AsterService;
use log::info;

#[tokio::main]
async fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    info!("Starting Aster");

    let _messager = common::messaging::crossbeam::CrossbeamMessagingFactory::default();

    // Create and init here
    info!("Initializing services");
    let mut services: Vec<Box<dyn AsterService>> = vec![];

    let mut handles = vec![];

    info!("Running services");
    for service in services.iter_mut() {
        handles.push(service.run());
    }

    futures::future::join_all(handles).await;
}
