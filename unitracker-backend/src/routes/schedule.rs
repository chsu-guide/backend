use std::sync::Arc;

use apply::Apply;
use axum::Json;
use axum::extract::State;
use axum::http::StatusCode;
use axum::{Router, extract::Query, routing::get};
use chrono::naive::serde::ts_seconds_option;
use chrono::{NaiveDateTime, Utc};
use serde::Deserialize;
use unitracker_psql::models::class::Class;
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
) -> Result<Json<Vec<Class>>, StatusCode> {
    context
        .database()
        .select_class_by_group_with_timestamps(
            query.group,
            query.start.unwrap_or(Utc::now().naive_local()),
            query.end.unwrap_or(Utc::now().naive_local()),
        )
        .await
        .map_err(|_| StatusCode::IM_A_TEAPOT)?
        .apply(|v| Ok(Json(v)))
}

#[derive(PartialEq, Eq, Deserialize, Debug)]
struct TeacherScheduleQuery {
    teacher: IdOrName,
    start: NaiveDateTime,
    end: NaiveDateTime,
}
async fn get_teacher_schedule(Query(schedule): Query<TeacherScheduleQuery>) {
    println!("{schedule:?}")
}

/// Maps /schedule, /schedule/teacher
pub fn schedule_router() -> Router<Arc<Context>> {
    Router::new()
        .route("/schedule", get(get_schedule))
        .route("/schedule/teacher", get(get_teacher_schedule))
}
