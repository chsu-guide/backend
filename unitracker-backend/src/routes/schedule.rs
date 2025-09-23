use axum::extract::Query;
use chrono::NaiveDateTime;
use serde::Deserialize;

use crate::util::types::IdOrName;

#[derive(Deserialize)]
struct ScheduleQuery {
    group: IdOrName,
    start: NaiveDateTime,
    end: NaiveDateTime,
}

async fn get_schedule(Query(schedule): Query<ScheduleQuery>) {}

#[derive(Deserialize)]
struct TeacherScheduleQuery {
    teacher: IdOrName,
    start: NaiveDateTime,
    end: NaiveDateTime,
}
async fn get_teacher_schedule(Query(schedule): Query<TeacherScheduleQuery>) {
    todo!()
}
