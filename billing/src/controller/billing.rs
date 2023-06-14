use std::{net::SocketAddr, sync::Arc};

use crate::graphql_schemas::{BillingSchema, MutationRoot, QueryRoot};
use async_graphql::{http::GraphiQLSource, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::{self, IntoResponse},
    routing::get,
    Router, Server,
};
use common::monitoring::readiness_handler;
use log::info;

use crate::services::database::DatabaseService;

pub struct BillingController {
    pub http_address: SocketAddr,
    pub readiness_endpoint: String,
    pub database_service: Arc<DatabaseService>,
}

async fn graphql_handler(schema: Extension<BillingSchema>, req: GraphQLRequest) -> GraphQLResponse {
    schema.execute(req.into_inner()).await.into()
}

async fn graphiql() -> impl IntoResponse {
    response::Html(GraphiQLSource::build().endpoint("/").finish())
}

impl BillingController {
    pub async fn new(
        http_address: SocketAddr,
        readiness_endpoint: String,
    ) -> Result<Self, anyhow::Error> {
        Ok(Self {
            http_address,
            readiness_endpoint,
            database_service: Arc::new(DatabaseService::new().await?),
        })
    }

    pub async fn start(&self) -> Result<(), anyhow::Error> {
        info!("Starting GraphQL server at {}", self.http_address);

        let schema = Schema::new(
            QueryRoot {
                database_service: self.database_service.clone(),
            },
            MutationRoot {
                database_service: self.database_service.clone(),
            },
            EmptySubscription,
        );

        let app = Router::new()
            .route("/", get(graphiql).post(graphql_handler))
            .route(self.readiness_endpoint.as_str(), get(readiness_handler))
            .layer(Extension(schema));

        let sarting_server_log_msg =
            &format!("Starting Billing GraphiQL IDE at {}", self.http_address);
        info!("{}", sarting_server_log_msg);

        Server::bind(&self.http_address)
            .serve(app.into_make_service())
            .await
            .unwrap();

        Ok(())
    }
}
