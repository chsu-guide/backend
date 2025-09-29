use axum::Router;

use crate::routes::schedule::get_schedule_router;
pub mod context;
pub mod routes;
pub mod util;
#[tokio::main]
async fn main() {
    let app = Router::new().merge(get_schedule_router());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Hello, world!");
    axum::serve(listener, app).await.unwrap();
}
