use axum::{Router, response::Html, routing::get};
use tracing::info;

pub async fn health() -> Html<&'static str> {
    info!("Healthy!");
    Html("Healthy")
}

pub fn health_router() -> Router<()> {
    Router::new().route("/health", get(health))
}
