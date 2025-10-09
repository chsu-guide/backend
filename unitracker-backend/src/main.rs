use std::sync::Arc;

use axum::{
    Router,
    extract::{MatchedPath, Request},
    middleware,
    response::Response,
};
use tower_http::trace::{DefaultOnRequest, DefaultOnResponse, TraceLayer};
use tracing::{info, info_span};
use tracing_subscriber::layer::SubscriberExt;
use unitracker_server::context::{Context, ContextParameters};

use crate::routes::{health::health_router, mixed, schedule, statistics};

pub mod routes;
#[tokio::main]
async fn main() {
    tracing_subscriber::fmt().init();

    info!("Started up");
    let app = Router::new()
        .merge(schedule::schedule_router())
        .merge(statistics::statistics_router())
        .merge(mixed::mixed_router())
        .with_state(Arc::new(
            Context::init(ContextParameters {
                connection_string: "postgres://unitracker:unitracker@127.0.0.1:3535/unitracker-db"
                    .to_string(),
            })
            .await,
        ))
        .merge(health_router())
        .layer(middleware::from_fn(log_request));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4321").await.unwrap();
    info!("Created a listener at {:?}", listener.local_addr());

    axum::serve(listener, app).await.unwrap();
}

async fn log_request(req: Request, next: axum::middleware::Next) -> Response {
    info!("Request: {}", req.uri());

    let response = next.run(req).await;

    info!("Response: {}", response.status());

    response
}
