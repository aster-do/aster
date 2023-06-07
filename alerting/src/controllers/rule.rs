use std::{net::SocketAddr, sync::Arc};

use anyhow::anyhow;
use async_graphql::{http::GraphiQLSource, EmptySubscription, Object, Schema, ID};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
    extract::Extension,
    response::{self, IntoResponse},
    routing::get,
    Router, Server,
};
use common::monitoring::readiness_handler;
use log::info;

use crate::{
    models::{dtos::alerting_rule::AlertingRuleDTO, input::alerting_rule::AlertingRuleInput},
    services::rule::RuleService,
};

#[derive(Debug)]
pub struct RuleController {
    //Config & stateful info
    rule_service: Arc<RuleService>,
    http_address: SocketAddr,
    readiness_endpoint: String,
}

impl RuleController {
    pub async fn new(
        http_address: SocketAddr,
        readiness_endpoint: String,
    ) -> Result<Self, anyhow::Error> {
        Ok(Self {
            //Config & stateful info
            rule_service: Arc::new(RuleService::new().await?),
            http_address,
            readiness_endpoint,
        })
    }

    pub async fn start(&self) -> Result<(), anyhow::Error> {
        let schema = Schema::new(
            QueryRule {
                rule_service: self.rule_service.clone(),
            },
            MutationRule {
                rule_service: self.rule_service.clone(),
            },
            EmptySubscription,
        );

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
        schema: Extension<Schema<QueryRule, MutationRule, EmptySubscription>>,
        req: GraphQLRequest,
    ) -> GraphQLResponse {
        schema.execute(req.into_inner()).await.into()
    }
    async fn graphiql() -> impl IntoResponse {
        response::Html(GraphiQLSource::build().endpoint("/").finish())
    }
}

struct QueryRule {
    //Config & stateful info
    rule_service: Arc<RuleService>,
}
#[Object]
impl QueryRule {
    async fn rule(&self, id: ID) -> Result<AlertingRuleDTO, anyhow::Error> {
        let rule = self
            .rule_service
            .get_rule(id.to_string())
            .await?
            .ok_or_else(|| anyhow!("Rule not found"))?;

        Ok(rule.into())
    }

    async fn rules(&self) -> Result<Vec<AlertingRuleDTO>, anyhow::Error> {
        let rules = self.rule_service.get_rules().await?;

        Ok(rules.into_iter().map(|r| r.into()).collect())
    }
}

struct MutationRule {
    //Config & stateful info
    rule_service: Arc<RuleService>,
}

#[Object]
impl MutationRule {
    async fn rule(&self, rule: AlertingRuleInput) -> Result<AlertingRuleDTO, anyhow::Error> {
        let rule = self
            .rule_service
            .mutate_rule(rule.into())
            .await
            .map_err(|e| anyhow!("Error creating rule: {}", e))?;

        Ok(rule.into())
    }

    async fn delete_rule(&self, id: ID) -> Result<bool, anyhow::Error> {
        self.rule_service
            .delete_rule(id.to_string())
            .await
            .map_err(|e| anyhow!("Error deleting rule: {}", e))?;

        Ok(true)
    }
}

//Config & stateful info
