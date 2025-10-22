use std::sync::Arc;

use axum::{Router, response::Html, routing::get};
use tracing::info;
use unitracker_server::context::Context;

pub async fn health() -> Html<&'static str> {
    info!("Healthy!");
    Html("Healthy")
}

/// Map /health
pub fn health_router() -> Router<Arc<Context>> {
    Router::new().route("/health", get(health))
}
