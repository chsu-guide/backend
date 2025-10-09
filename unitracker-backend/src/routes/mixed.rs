use std::sync::Arc;

use axum::{
    Json, Router,
    extract::{Query, State},
    http::StatusCode,
    routing::get,
};
use chrono::NaiveDateTime;
use serde::Deserialize;

use unitracker_psql::models::class::Class;
use unitracker_server::context::Context;
use unitracker_types::IdOrName;

#[derive(Deserialize)]
struct MixedQuery {
    pub auditorium: Option<IdOrName>,
    pub teacher: Option<IdOrName>,
    pub group: Option<IdOrName>,
    pub discipline: Option<IdOrName>,
    pub from: Option<NaiveDateTime>,
    pub to: Option<NaiveDateTime>,
}

async fn get_mixed_schedule(
    State(ctx): State<Arc<Context>>,
    Query(mixed): Query<MixedQuery>,
) -> Result<Json<Vec<Class>>, StatusCode> {
    let schedule = ctx
        .database()
        .get_mixed_schedule(
            mixed.auditorium,
            mixed.teacher,
            mixed.group,
            mixed.discipline,
            mixed.from,
            mixed.to,
        )
        .await
        .map_err(|e| StatusCode::IM_A_TEAPOT)?;
    return Ok(Json(schedule));
}

pub fn mixed_router() -> Router<Arc<Context>> {
    Router::new().route("/mixed", get(get_mixed_schedule))
}
