use std::sync::Arc;

use axum::Router;
use eyre::Context as _;
use tokio::signal;
use tracing::{Level, info};
use tracing_subscriber::{filter, layer::SubscriberExt, util::SubscriberInitExt};
use unitracker_server::context::{Context, ContextParameters};

use crate::routes::{
    auditorium,
    health::{self},
    metadata, mixed, schedule, statistics,
};

pub mod routes;
#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenvy::dotenv().unwrap();
    let filter = filter::Targets::new()
        .with_target("unitracker-psql", Level::TRACE)
        .with_target("sqlx", Level::INFO)
        .with_target("reqwest", Level::INFO)
        .with_target("axum", Level::INFO)
        .with_target("hyper_util", Level::INFO)
        .with_default(Level::TRACE);

    let layer = tracing_subscriber::fmt::layer();
    tracing_subscriber::registry()
        .with(layer)
        .with(filter)
        .init();

    info!("Started up");
    let app = Router::new()
        .merge(schedule::schedule_router())
        .merge(statistics::statistics_router())
        .merge(auditorium::get_auditorium_router())
        .merge(mixed::mixed_router())
        .merge(health::health_router())
        .merge(metadata::metadata_router())
        .with_state(Arc::new(
            Context::init(ContextParameters {
                connection_string: "postgres://unitracker:unitracker@127.0.0.1:3535/unitracker-db"
                    .to_string(),
            })
            .await,
        ));

    let listener = tokio::net::TcpListener::bind("0.0.0.0:4321").await.unwrap();
    info!(
        "Created a listener at {:?}",
        listener.local_addr().wrap_err("Failed to listen").unwrap()
    );

    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal())
        .await
        .wrap_err("Failed to serve")
}

async fn shutdown_signal() {
    let ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install ctrl+c handler");
    };

    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    tokio::select! {
        _ = ctrl_c => {
            tracing::error!("Received C-c, shutting down");
        },
        _ = terminate => {
            tracing::error!("Received SIGTERM, shutting down");
        },
    }
}
