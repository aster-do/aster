#### cargo.toml

```toml
async-graphql = { version = "5.0.9", features = ["dynamic-schema"]}
async-graphql-axum = "5.0.9"
axum = "0.6.18"
tokio = { version = "1.28.2", features = ["macros", "rt-multi-thread"]}
```

#### main.rs

```rust

use async_graphql::{http::GraphiQLSource, EmptyMutation, EmptySubscription, Schema};
use async_graphql_axum::{GraphQLRequest, GraphQLResponse};
use axum::{
extract::Extension,
response::{self, IntoResponse},
routing::get,
Router, Server,
};

struct QueryRoot;

#[async_graphql::Object]
impl QueryRoot {
async fn alerts(&self) -> String {
"Alerts".to_owned()
}

    async fn alert(&self, id: i32) -> String {
        format!("Alert {}", id)
    }

    async fn notification(&self, id: i32) -> String {
        format!("Notification {}", id)
    }

    async fn notifications(&self) -> String {
        "Notifications".to_owned()
    }

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

#[tokio::main]
async fn main() {
let schema = Schema::new(QueryRoot, EmptyMutation, EmptySubscription);

    let app = Router::new()
        .route("/", get(graphiql).post(graphql_handler))
        .layer(Extension(schema));

    println!("GraphiQL IDE: http://localhost:8000");

    Server::bind(&"127.0.0.1:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

}
```
