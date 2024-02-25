use unitracker_chsu::request::*;
use unitracker_chsu::request::groups::get_groups;
use unitracker_chsu::request::schedule::{get_schedule, ScheduleRequestBuilder, ScheduleType};
use chrono::{Days, NaiveDate, Utc};
use unitracker_chsu::request::teachers::get_teachers;

#[tokio::main]
async fn main() {
    let resp = auth::get_auth().await;
    let bearer = match resp {
        Ok(s) => s,
        Err(AuthErrors::IncorrectAuthData) => panic!("Auth broke"),
        _ => panic!("????")
    };
    let test = get_groups(&bearer.data).await.unwrap();
    let id = test.iter().find(|x| x.title == "1ПИб-02-3оп-22").unwrap();
    let schedule_req = ScheduleRequestBuilder::new()
        .start(Utc::now().date_naive())
        .end(Utc::now().checked_add_days(Days::new(7)).unwrap().date_naive())
        .schedule_type(ScheduleType::Group(id.clone()))
        .build();
    let schedule = get_schedule(&bearer.data, schedule_req).await.unwrap();

    println!("{:?}", schedule)
}
