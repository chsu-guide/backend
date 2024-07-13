use std::env;
use chrono::{Days, Months, NaiveDate, TimeZone, Utc};
use dotenv;
use unitracker_chsu::model::teachers::Teacher;
use unitracker_chsu::request::auth::get_auth;
use unitracker_chsu::request::schedule::{get_schedule, get_school_week, ScheduleRequestBuilder, ScheduleType};
use unitracker_postgres::database::Database;

#[tokio::main]
async fn main() {
    dotenv::dotenv().expect("TODO: panic message");

    let token = get_auth().await.unwrap();
    let bearer = &token.data;
    // let week = get_school_week(bearer, Utc::now().naive_utc().date()).await;
    let url = env::var("DATABASE_URL").unwrap();
    let db = Database::new(&url).unwrap();
    let mut ershov = Teacher::default();
    ershov.id = 1396414847681304437;
    let rq = ScheduleRequestBuilder::new()
        .start(Utc.with_ymd_and_hms(2024, 5, 26, 0, 0, 0).unwrap().naive_utc().date())
        .end(Utc.with_ymd_and_hms(2024, 5, 26, 1, 0, 0).unwrap().naive_utc().date())
        .schedule_type(ScheduleType::Lecturer(ershov))
        .build();
    println!("rq: {rq}");
    // let auds = get_schedule(bearer, rq).await.unwrap();
    // println!("{auds:?}");
    // let id = auds[1].id;
    // for i in auds {
    //     let teacher_id = i.lecturers.clone().unwrap().iter().map(|x| x.id).collect::<Vec<i64>>();
    //     let group_id = i.groups.iter().map(|x| x.id).collect::<Vec<i64>>();
    //     let auditorium_id = i.clone().auditory.unwrap().id;
    //     let aud = db
    //         .insert_schedule(
    //             i.into(),
    //             auditorium_id,
    //             group_id.as_slice(),
    //             teacher_id.as_slice())
    //         .await
    //         .unwrap();
    // }
    // let output = db.select_schedule(id).await.unwrap();
}
