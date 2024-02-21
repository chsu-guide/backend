// use unitracker_chsu::request::schedule;
// use unitracker_chsu::request::schedule::{RequestType, ScheduleRequest, ScheduleRequestBuilder};

#[tokio::main]
async fn main() {
    let resp = unitracker_chsu::request::auth::get_auth().await;
    println!("{resp:?}");
}
