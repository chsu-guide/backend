use unitracker_chsu::request::schedule;
use unitracker_chsu::request::schedule::{RequestType, ScheduleRequest, ScheduleRequestBuilder};

#[tokio::main]
async fn main() {
    let test_request: ScheduleRequest = ScheduleRequestBuilder::new()
        .request_type(RequestType::Student)
        .group_id("1739582424505775711")
        .teacher_id("1472314025600620405")
        .start("06.02.2024".into())
        .end("06.02.2024".into())
        .build()
        .unwrap();
    dbg!(&test_request.form_schedule_url());
    let w = schedule::get_schedule(test_request).await.unwrap();
    dbg!(w);
}
