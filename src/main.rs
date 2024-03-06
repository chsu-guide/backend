#![feature(iter_intersperse)]

use std::env;
use std::str::from_utf8;
use chrono::{Days, NaiveDate, Utc};
use unitracker_chsu::request::buildings::get_buildings;
use sqlx::query;
use dotenv;
use sqlx::postgres::PgPool;
use unitracker_chsu::model;
use unitracker_chsu::model::schedule::Group;
use unitracker_chsu::request::auditoriums::get_auditoriums;
use unitracker_chsu::request::auth::get_auth;
use unitracker_chsu::request::schedule::{get_schedule, ScheduleRequestBuilder, ScheduleType};
use unitracker_chsu::request::teachers::get_teachers;
use unitracker_postgres::database::Database;

#[tokio::main]
async fn main() {
    dotenv::dotenv();

    let token = get_auth().await.unwrap();
    let bearer = &token.data;
    let url = env::var("DATABASE_URL").unwrap();
    let db = Database::new(&url).unwrap();
    let rq = ScheduleRequestBuilder::new()
        .start(Utc::now().naive_utc().date())
        .end(Utc::now().naive_utc().date())
        .schedule_type(ScheduleType::Full)
        .build();
    let auds = get_schedule(bearer, rq).await.unwrap();
    let id = auds[1].id;
    for i in auds {
        let teacher_id = i.lecturers.clone().unwrap().iter().map(|x| x.id).collect::<Vec<i64>>();
        let group_id = i.groups.iter().map(|x| x.id).collect::<Vec<i64>>();
        let auditorium_id = i.clone().auditory.unwrap().id;
        let aud = db
            .insert_schedule(
                i.into(),
                auditorium_id,
                group_id.as_slice(),
                teacher_id.as_slice())
            .await
            .unwrap();
    }
    let output = db.select_schedule(id).await.unwrap();

    println!("{:?}", output.unwrap())
}
