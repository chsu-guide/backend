mod update;

use std::env;
use chrono::{NaiveDate, TimeZone, Utc};
use dotenv;
use reqwest::ClientBuilder;
use unitracker_chsu::request::auth::get_auth;
use unitracker_chsu::request::{RequestErrors, SharedErrors};
use unitracker_chsu::request::schedule::{get_schedule, ScheduleRequestBuilder, ScheduleType};
use unitracker_postgres::database::Database;

#[tokio::main]
async fn main() -> Result<(), RequestErrors> {
    dotenv::dotenv().expect("TODO: panic message");

    let mut client = ClientBuilder::new().user_agent("").build()?;

    let auth = get_auth(&mut client).await.unwrap();

    let db_url = env::var("DATABASE_URL").unwrap();

    let db = Database::new(&db_url).unwrap();

    let schedule_request = ScheduleRequestBuilder::new()
        .start(NaiveDate::from_ymd_opt(2024, 4, 1).unwrap())
        .end(NaiveDate::from_ymd_opt(2024, 4, 30).unwrap())
        .schedule_type(ScheduleType::Full)
        .build();
    let mut group: i64 = 0;
    if let Ok(schedule) = get_schedule(&mut client, &auth.data, schedule_request).await {
        for class in schedule {
            let db_class = class.clone().into();
            group = class.groups.first().unwrap().id;
            db.insert_schedule(db_class,
                               class.id,
                               &class.lecturers.unwrap().iter().map(|x| x.id).collect::<Vec<i64>>(),
                               &class.groups.iter().map(|x| x.id).collect::<Vec<i64>>())
                .await
                .expect("TODO: panic message");
        }
    }

    let db_sched = db.select_schedule(group).await;
    println!("{db_sched:?}");
    Ok(())

}
