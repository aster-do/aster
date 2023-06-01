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
use common::{messaging::MessagingFactory, services::AsterService};
use log::info;

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
    async fn init(&mut self, _: &mut dyn MessagingFactory) {
        info!("Initializing the billing service...");
        // TODO
    }

    async fn run(&mut self) {
        info!("Starting the billing service...");
        run().await;
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

    info!("GraphiQL IDE: http://localhost:8000");

    Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
