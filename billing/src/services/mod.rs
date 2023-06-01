use crate::schemas::{BillingSchema, QueryRoot};
use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use async_trait::async_trait;
use axum::{
    extract::Extension,
    response::{self, IntoResponse},
    routing::get,
    Router, Server,
};
use common::{messaging::crossbeam::CrossbeamMessagingFactory, services::AsterService};
use log::info;

const SERVICE_PORT: u16 = 3033;

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
    let schema = Schema::new(QueryRoot, EmptyMutation, EmptySubscription);

    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(Extension(schema));

    let sarting_server_log_msg = &format!("GraphiQL IDE: http://0.0.0.0:{}", SERVICE_PORT);
    info!("{}", sarting_server_log_msg);

    Server::bind(&format!("0.0.0.0:{}", SERVICE_PORT).parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
