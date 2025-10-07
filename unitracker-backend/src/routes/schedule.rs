use std::sync::Arc;

use crate::context::{ApplicationContext, create_context};
use axum::extract::State;
use axum::extract::rejection::QueryRejection;
use axum::{Router, extract::Query, routing::get};
use chrono::NaiveDateTime;
use chrono::naive::serde::ts_seconds_option;
use serde::Deserialize;
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
    State(context): State<Arc<ApplicationContext>>,
    schedule: Result<Query<ScheduleQuery>, QueryRejection>,
) {
    println!("{schedule:?}");
    let schedule = schedule.unwrap().0;
    match schedule.group {
        IdOrName::Id(id) => todo!(),
        IdOrName::Name(name) => {
            let schedule_results = context
                .database()
                .select_class_by_name_with_timestamps(
                    name,
                    schedule.start.unwrap_or_default(),
                    schedule.end.unwrap_or_default(),
                )
                .await;
            let data = println!("{schedule_results:?}");
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

pub fn get_schedule_router() -> Router {
    Router::new()
        .route("/schedule", get(get_schedule))
        .route("/schedule/teacher", get(get_teacher_schedule))
        .with_state(create_context(
            "postgres://unitracker:unitracker@127.0.0.1:3535/unitracker-db",
        ))
}
