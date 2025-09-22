use axum::extract::Query;
use chrono::NaiveDateTime;
use serde::Deserialize;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum IdOrName {
    Id(i64),
    Name(String),
}
#[derive(Deserialize)]
struct ScheduleQuery {
    group: IdOrName,
    start: NaiveDateTime,
    end: NaiveDateTime,
}

async fn get_schedule(Query(schedule): Query<ScheduleQuery>) {
    todo!()
}

#[derive(Deserialize)]
struct TeacherScheduleQuery {
    teacher: IdOrName,
    start: NaiveDateTime,
    end: NaiveDateTime,
}
async fn get_teacher_schedule(Query(schedule): Query<TeacherScheduleQuery>) {
    todo!()
}
