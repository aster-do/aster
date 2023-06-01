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
use log::info;

use crate::services::channel::ChannelService;

pub struct ChannelController {
    //Config & stateful info
    _channel_service: ChannelService,
    http_address: SocketAddr,
}

impl ChannelController {
    pub fn new(http_address: SocketAddr) -> Result<Self> {
        Ok(Self {
            //Config & stateful info
            _channel_service: ChannelService::new()?,
            http_address,
        })
    }

    pub async fn start(&self) -> Result<()> {
        let schema = Schema::new(QueryRoot, EmptyMutation, EmptySubscription);

        let app = Router::new()
            .route("/", get(Self::graphiql).post(Self::graphql_handler))
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
    async fn channels(&self) -> String {
        //TODO Call the channel service
        "Channels".to_owned()
    }
    async fn channel(&self, id: i32) -> String {
        //TODO Call the channel service
        format!("Channel {}", id)
    }
}
