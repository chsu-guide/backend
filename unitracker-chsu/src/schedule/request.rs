use std::collections::HashMap;
use base64::Engine;
use tokio::io::AsyncWriteExt;
use crate::schedule::models::{schedule::Week, students::Students};
use serde_derive;
use serde;
use reqwest;
/// # Request logic:
///
/// ## Link:
///
/// https://www.chsu.ru/raspisanie/cache
///
/// ## base64-encoded parameters:
///
/// WyJzdHVkZW50IiwiMTczOTU4MjQyNDUwNTc3NTcxMSIsbnVsbCwiMDYuMDIuMjAyNCIsIjA2LjAyLjIwMjQiXQ => ["student","1739582424505775711",null,"06.02.2024","06.02.2024"]
/// WyJ0dXRvciIsIjE3Mzk1ODI0MjQ1MDU3NzU3MTEiLCIxNDcyMzE0MDI1NjAwNjIwNDA1IiwiMDguMDIuMjAyNCIsIjA4LjAyLjIwMjQiXQ => ["tutor","1739582424505775711","1472314025600620405","08.02.2024","08.02.2024"]
///
/// ## Argument format
/// type ("student"/"tutor"),
/// group ID (nullable, stays at whichever was last requested for tutor request),
/// tutor ID (nullable, stays at whichever was last requested for student request),
/// start date (dd.MM.yyyy),
/// end date (dd.MM.yyyy)
///
/// _=.json to specify the format
///
/// ### Example:
///
/// https://www.chsu.ru/raspisanie/cache/WyJzdHVkZW50IiwiMTczOTU4MjQyNDUwNTc3NTcxMSIsbnVsbCwiMDYuMDIuMjAyNCIsIjA2LjAyLjIwMjQiXQ_=.json?1707052923221
pub async fn get_weeks() -> serde_json::Result<Vec<Week>> {
    let schedule_unparsed = get_schedule().await.unwrap();
    let schedule_req = serde_json::from_str::<Vec<Week>>(&schedule_unparsed);
    schedule_req
}
async fn get_schedule() -> reqwest::Result<String> {
    let url = form_schedule_url("student", "1739582424505775711", "", "06.02.2024","29.02.2024");
    let res = reqwest::get(url).await?.text().await?;
    Ok(res)
}
async fn get_groups() -> reqwest::Result<HashMap<String, String>> {
    let url = "https://www.chsu.ru/raspisanie/cache/student.json?";
    let res = reqwest::get(url).await?.text().await?;
    let res_format = serde_json::from_str::<Vec<Students>>(&res).unwrap();
    let mut map: HashMap<String, String> = Default::default();
    for student in res_format {
        map.insert(student.code, student.group);
    }
    Ok(map)
}
fn form_schedule_url(request_type: &str, group_id: &str, teacher_id: &str, start: &str, end: &str) -> reqwest::Url {
    let request_string = format!("[\"{}\",\"{}\",null,\"{}\",\"{}\"]", request_type, group_id, start, end);
    let b64_req = base64::engine::general_purpose::STANDARD.encode(request_string.clone());
    let url = format!(
        "https://www.chsu.ru/raspisanie/cache/{}.json",
        b64_req
    );
    let url = url.parse::<reqwest::Url>().unwrap();
    url
}

#[cfg(test)]
mod request_tests {
    use crate::schedule::request::form_schedule_url;
    #[test]
    fn test_schedule_url() {
        let test_url = form_schedule_url("student", "1739582424505775711", "", "06.02.2024","29.02.2024");
        let url = "https://www.chsu.ru/raspisanie/cache/WyJzdHVkZW50IiwiMTczOTU4MjQyNDUwNTc3NTcxMSIsbnVsbCwiMDYuMDIuMjAyNCIsIjI5LjAyLjIwMjQiXQ==.json".parse::<reqwest::Url>().unwrap();
        assert_eq!(test_url, url);
    }
}