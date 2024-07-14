// use std::env;
// use chrono::{Utc, Days};
// use reqwest::Client;
// use clokwerk::{AsyncScheduler, TimeUnits, Job, Interval::*};
// use dotenv::dotenv;
// use unitracker_chsu::request::auth::get_auth;
// use unitracker_chsu::request::SharedErrors;
// use unitracker_chsu::request::schedule::{get_schedule, ScheduleRequest, ScheduleRequestBuilder, ScheduleType};
// use unitracker_postgres::database::Database;
//
// pub async fn update_loop(mut client: Client) -> Result<(), SharedErrors> {
//     let client_ref = &mut client;
//     let env = dotenv::dotenv();
//     let mut scheduler = AsyncScheduler::new();
//
//     let db_url = env::var("DATABASE_URL").unwrap();
//
//     let db = Database::new(&db_url).unwrap();
//
//     let auth = get_auth(client_ref).await?;
//
//     // Full schedule check
//     scheduler.every(7.days()).run(|| async {
//         let req = ScheduleRequestBuilder::new()
//             .schedule_type(ScheduleType::Full)
//             .start(Utc::now().naive_utc().date())
//             .end(Utc::now().checked_add_days(Days(7)).unwrap().naive_utc().date())
//             .build();
//         let classes = get_schedule(client_ref, &auth.data, req).await?;
//         todo!()
//     })
// }