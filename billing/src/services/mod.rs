use crate::schemas::{BillingSchema, MutationRoot, QueryRoot};
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use async_trait::async_trait;
use axum::{
    extract::Extension,
    response::{self, IntoResponse},
    routing::get,
    Router, Server,
};
use common::{
    messaging::tokio_broadcast::CrossbeamMessagingFactory, monitoring::readiness_handler,
    services::AsterService,
};
use log::info;

const SERVICE_PORT: u16 = 3033;
const READINESS_SERVER_ENDPOINT: &str = "/health";

pub struct BillingService;

impl BillingService {
    pub fn new() -> Self {
        BillingService
    }
}

impl Default for BillingService {
    fn default() -> Self {
        BillingService::new()
    }
}

#[async_trait]
impl AsterService for BillingService {
    async fn init(&mut self, _: &mut CrossbeamMessagingFactory) -> Result<(), anyhow::Error> {
        info!("Initializing the billing service...");
        // TODO

        Ok(())
    }

    async fn run(&mut self) -> Result<(), anyhow::Error> {
        info!("Starting the billing service...");
        run().await;

        Ok(())
    }
}

async fn graphql_handler(schema: Extension<BillingSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

pub async fn run() {
    let schema = Schema::new(QueryRoot, MutationRoot, EmptySubscription);

    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .route(READINESS_SERVER_ENDPOINT, get(readiness_handler))
        .layer(Extension(schema));

    let sarting_server_log_msg = &format!("GraphiQL IDE: http://0.0.0.0:{}", SERVICE_PORT);
    info!("{}", sarting_server_log_msg);

    Server::bind(&format!("0.0.0.0:{}", SERVICE_PORT).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
