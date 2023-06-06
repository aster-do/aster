use std::net::SocketAddr;

use anyhow::{Ok, Result};

use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::{self, IntoResponse},
    routing::get,
    Router, Server,
};
use common::monitoring::readiness_handler;
use log::info;

use crate::services::rule::RuleService;

#[derive(Debug)]
pub struct RuleController {
    //Config & stateful info
    _rule_service: RuleService,
    http_address: SocketAddr,
    readiness_endpoint: String,
}

impl RuleController {
    pub async fn new(http_address: SocketAddr, readiness_endpoint: String) -> Result<Self> {
        Ok(Self {
            //Config & stateful info
            _rule_service: RuleService::new().await,
            http_address,
            readiness_endpoint,
        })
    }

    pub async fn start(&self) -> Result<()> {
        let schema = Schema::new(QueryRoot, EmptyMutation, EmptySubscription);

        let app = Router::new()
            .route("/", get(Self::graphiql).post(Self::graphql_handler))
            .route(&self.readiness_endpoint, get(readiness_handler))
            .layer(Extension(schema));

        info!("Starting GraphQL server at {}", self.http_address);

        Server::bind(&self.http_address)
            .serve(app.into_make_service())
            .await?;

        Ok(())
    }

    async fn graphql_handler(
        schema: Extension<Schema<QueryRoot, EmptyMutation, EmptySubscription>>,
        req: GraphQLRequest,
    ) -> GraphQLResponse {
        schema.execute(req.into_inner()).await.into()
    }
    async fn graphiql() -> impl IntoResponse {
        response::Html(GraphiQLSource::build().endpoint("/").finish())
    }
}

struct QueryRoot;
#[async_graphql::Object]
impl QueryRoot {
    async fn rules(&self) -> String {
        //TODO Call rule service
        "Rules".to_owned()
    }
    async fn rule(&self, id: i32) -> String {
        //TODO Call rule service
        format!("Rule {}", id)
    }
}

//Config & stateful info
