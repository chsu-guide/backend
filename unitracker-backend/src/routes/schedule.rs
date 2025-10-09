use std::sync::Arc;

use axum::Json;
use axum::extract::State;
use axum::extract::rejection::QueryRejection;
use axum::http::StatusCode;
use axum::{Router, extract::Query, routing::get};
use chrono::NaiveDateTime;
use chrono::naive::serde::ts_seconds_option;
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
    schedule: Result<Query<ScheduleQuery>, QueryRejection>,
) -> Result<Json<Vec<Class>>, StatusCode> {
    println!("{schedule:?}");
    let schedule = schedule.unwrap().0;
    match schedule.group {
        IdOrName::Id(id) => {
            let schedule_result = context
                .database()
                .select_class_by_group_with_timestamps(
                    id,
                    schedule.start.unwrap(),
                    schedule.end.unwrap(),
                )
                .await
                .map_err(|e| StatusCode::IM_A_TEAPOT)?;
            println!("{schedule_result:?}");
            return Ok(Json(schedule_result));
        }
        IdOrName::Name(name) => {
            let schedule_results = context
                .database()
                .select_class_by_name_with_timestamps(
                    name,
                    schedule.start.unwrap_or_default(),
                    schedule.end.unwrap_or_default(),
                )
                .await
                .map_err(|e| StatusCode::IM_A_TEAPOT);
            let data = println!("{schedule_results:?}");
            return Ok(Json(schedule_results?));
        }
    }
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

pub fn schedule_router() -> Router<Arc<Context>> {
    Router::new()
        .route("/schedule", get(get_schedule))
        .route("/schedule/teacher", get(get_teacher_schedule))
}
