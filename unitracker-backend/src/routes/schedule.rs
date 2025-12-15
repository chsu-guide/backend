use std::sync::Arc;

use apply::Apply;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{Router, extract::Query, routing::get};
use chrono::naive::serde::ts_seconds_option;
use chrono::{NaiveDateTime, Utc};
use eyre::Context as _;
use serde::Deserialize;
use tracing::warn;
use unitracker_psql::models::class::Class;
use unitracker_psql::models::dtos;
use unitracker_server::context::Context;
use unitracker_types::IdOrName;

#[derive(PartialEq, Eq, Deserialize, Debug)]
struct ScheduleQuery {
    group: IdOrName,
    #[serde(with = "ts_seconds_option")]
    start: Option<NaiveDateTime>,
    #[serde(with = "ts_seconds_option")]
    end: Option<NaiveDateTime>,
}

async fn get_schedule(
    State(context): State<Arc<Context>>,
    Query(query): Query<ScheduleQuery>,
) -> Result<Json<Vec<dtos::class::Class>>, StatusCode> {
    context
        .database()
        .select_class_by_group_with_timestamps(
            query.group,
            query.start.unwrap_or(Utc::now().naive_local()),
            query.end.unwrap_or(Utc::now().naive_local()),
        )
        .await
        .wrap_err("Query failed")
        .inspect_err(|e| warn!("{e}"))
        .map_err(|_| StatusCode::IM_A_TEAPOT)?
        .apply(|v| Ok(Json(v)))
}

#[derive(PartialEq, Eq, Deserialize, Debug)]
struct TeacherScheduleQuery {
    teacher: IdOrName,
    #[serde(with = "ts_seconds_option")]
    start: Option<NaiveDateTime>,
    #[serde(with = "ts_seconds_option")]
    end: Option<NaiveDateTime>,
}
async fn get_teacher_schedule(
    State(context): State<Arc<Context>>,
    Query(query): Query<TeacherScheduleQuery>,
) -> Result<Json<Vec<dtos::class::Class>>, StatusCode> {
    context
        .database()
        .select_class_by_teacher_with_timestamps(
            query.teacher,
            query.start.unwrap_or(Utc::now().naive_local()),
            query.end.unwrap_or(Utc::now().naive_local()),
        )
        .await
        .wrap_err("Query failed")
        .inspect_err(|e| warn!("{e}"))
        .map_err(|_| StatusCode::IM_A_TEAPOT)?
        .apply(|v| Ok(Json(v)))
}

#[derive(PartialEq, Eq, Deserialize, Debug)]
struct AuditoriumScheduleQuery {
    auditorium: IdOrName,
    #[serde(with = "ts_seconds_option")]
    start: Option<NaiveDateTime>,
    #[serde(with = "ts_seconds_option")]
    end: Option<NaiveDateTime>,
}

async fn get_auditorium_schedule(
    State(context): State<Arc<Context>>,
    Query(query): Query<AuditoriumScheduleQuery>,
) -> Result<Json<Vec<dtos::class::Class>>, StatusCode> {
    context
        .database()
        .select_class_by_auditorium_with_timestamps(
            query.auditorium,
            query.start.unwrap_or(Utc::now().naive_local()),
            query.end.unwrap_or(Utc::now().naive_local()),
        )
        .await
        .wrap_err("Query failed")
        .inspect_err(|e| warn!("{e}"))
        .map_err(|_| StatusCode::IM_A_TEAPOT)?
        .apply(|v| Ok(Json(v)))
}

/// Maps /schedule, /schedule/teacher
pub fn schedule_router() -> Router<Arc<Context>> {
    Router::new()
        .route("/schedule", get(get_schedule))
        .route("/schedule/teacher", get(get_teacher_schedule))
        .route("/schedule/auditorium", get(get_auditorium_schedule))
}
