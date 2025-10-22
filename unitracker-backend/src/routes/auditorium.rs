use std::sync::Arc;

use apply::Apply;
use axum::{
    Json, Router,
    extract::{Query, State},
    http::StatusCode,
    routing::get,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use unitracker_psql::models::class::Class;
use unitracker_server::context::Context;
use unitracker_types::IdOrName;

#[derive(Deserialize, Debug)]
struct AuditoriumQuery {
    auditorium: IdOrName,
    #[serde(with = "chrono::naive::serde::ts_seconds")]
    start: NaiveDateTime,
    #[serde(with = "chrono::naive::serde::ts_seconds")]
    end: NaiveDateTime,
}

#[derive(Serialize, Deserialize, Debug)]
enum AvailabilityResponse {
    Free,
    Busy(Vec<BusyRange>),
}
#[derive(Serialize, Deserialize, Debug)]
struct BusyRange {
    start: NaiveDateTime,
    end: NaiveDateTime,
}

impl BusyRange {
    fn from_class(class: &Class) -> Self {
        Self {
            start: class.start_time,
            end: class.end_time,
        }
    }
}

/// Returns whether the auditorium is available during the specified time range
#[tracing::instrument]
async fn is_available(
    State(ctx): State<Arc<Context>>,
    Query(query): Query<AuditoriumQuery>,
) -> Result<Json<AvailabilityResponse>, StatusCode> {
    let auds = ctx
        .database()
        .auditorium_is_available(query.auditorium, query.start, query.end)
        .await
        .map_err(|_| StatusCode::IM_A_TEAPOT)?;
    match auds.is_empty() {
        true => AvailabilityResponse::Free,
        false => AvailabilityResponse::Busy(auds.iter().map(BusyRange::from_class).collect()),
    }
    .apply(Json)
    .apply(Ok)
}

// struct AuditoriumListQuery {
//     building: IdOrName,
//     floor: u8,
//     start: NaiveDateTime,
//     end: NaiveDateTime,
// }
// async fn get_availability(
//     State(ctx): State<Arc<Context>>,
//     Query(auditoriums): Query<AuditoriumListQuery>,
// ) {
// }
pub fn get_auditorium_router() -> Router<Arc<Context>> {
    Router::new().route("/auditorium/available", get(is_available))
}
