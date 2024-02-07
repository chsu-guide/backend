use unitracker_chsu::schedule::request;
use unitracker_chsu::schedule::request::{RequestType, ScheduleRequest, ScheduleRequestBuilder};

#[tokio::main]
async fn main() {
    let test_request: ScheduleRequest = ScheduleRequestBuilder::new()
        .request_type(RequestType::Student)
        .group_id("1739582424505775711")
        .teacher_id("1472314025600620405")
        .start("06.02.2024".into())
        .end("19.02.2024".into())
        .build()
        .unwrap();
    let r = request::get_weeks(test_request).await;
    dbg!(r);
}
