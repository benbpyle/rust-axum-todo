use aws_config::{
    imds::client, meta::region::RegionProviderChain, BehaviorVersion, Region, SdkConfig,
};
use aws_sdk_dynamodb::Client;
use axum::{
    routing::{get, post},
    Router,
};
use data::{models::AppState, services::services::TodoService};
use routes::{
    create_todo, delete_todo_by_id, find_todo_by_id, handler_404, health, update_todo_by_id,
};
mod data;
mod routes;

#[tokio::main]
async fn main() {
    // build our application with a single route

    let use_local = &std::env::var("USE_LOCAL");
    let region_provider = RegionProviderChain::default_provider().or_else("us-west-2");
    let config = aws_config::from_env().region(region_provider).load().await;
    let db_config = aws_sdk_dynamodb::config::Builder::from(&config).build();
    let mut dynamodb_client: Client = Client::from_conf(db_config);
    let table_name = std::env::var("TABLE_NAME").expect("TABLE_NAME must be set");

    if use_local.is_ok() {
        let dynamodb_local_config = aws_sdk_dynamodb::config::Builder::from(&config)
            .endpoint_url(
                // 8000 is the default dynamodb port
                //"http://localhost:8000",
                "http://host.docker.internal:8000",
            )
            .region(Region::from_static("us-east-1"))
            .build();
        dynamodb_client = Client::from_conf(dynamodb_local_config);
    }

    let shared_state = AppState {
        todo_service: TodoService::new(dynamodb_client, table_name.to_string()),
    };

    let app = app(shared_state);

    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("0.0.0.0:8080").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn app(app_state: AppState) -> Router {
    Router::new()
        .route("/", post(create_todo))
        .route(
            "/:id",
            get(find_todo_by_id)
                .put(update_todo_by_id)
                .delete(delete_todo_by_id),
        )
        .route("/health", get(health))
        .with_state(app_state)
        .fallback(handler_404)
}
