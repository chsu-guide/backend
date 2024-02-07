use base64::Engine;
use crate::schedule::models::{schedule::Week};
use reqwest::{Url};
use std::result::Result;
use thiserror::Error;
use url::ParseError;

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
/// https://www.chsu.ru/raspisanie/cache/WyJzdHVkZW50IiwiMTczOTU4MjQyNDUwNTc3NTcxMSIsbnVsbCwiMDYuMDIuMjAyNCIsIjA2LjAyLjIwMjQiXQ_=.json
pub async fn get_weeks(request: ScheduleRequest) -> Result<Vec<Week>, ScheduleError> {
    let schedule_unparsed = get_schedule(request).await?;
    let schedule_req = serde_json::from_str::<Vec<Week>>(&schedule_unparsed)?;
    Ok(schedule_req)
}
#[derive(Error, Debug)]
pub enum ScheduleError {
    #[error("Error while parsing url: {0}")]
    ParseError(#[from] url::ParseError),
    #[error("Error during http request: {0}")]
    ReqwestError(#[from] reqwest::Error),
    #[error("Error during deserialization: {0}")]
    SerdeError(#[from] serde_json::Error)
}
async fn get_schedule(request: ScheduleRequest) -> Result<String, ScheduleError> {
    let url = request.form_schedule_url()?;
    let res = reqwest::get(url).await?.text().await?;
    Ok(res)
}
// async fn get_groups() -> ReqwestResult<HashMap<String, String>> {
//     let url = "https://www.chsu.ru/raspisanie/cache/student.json?";
//     let res = reqwest::get(url).await?.text().await?;
//     let res_format = serde_json::from_str::<Vec<Students>>(&res).unwrap();
//     let mut map: HashMap<String, String> = Default::default();
//     for student in res_format {
//         map.insert(student.code, student.group);
//     }
//     Ok(map)
// }

/// Possible request types. Only two exist
#[derive(Debug, Eq, PartialEq)]
pub enum RequestType {
    Student,
    Teacher
}
/// A struct with parameters for [`get_schedule()`] request
#[derive(Debug, Eq, PartialEq)]
pub struct ScheduleRequest {
    request_type: RequestType,
    group_id: Option<String>,
    teacher_id: Option<String>,
    start: String,
    end: Option<String>
}
/// Possible errors emitted by [`ScheduleRequestBuilder`]
#[derive(Debug)]
pub enum ScheduleBuildErrors {
    NoRequestType,
    NoDate
}
/// Builder for [`ScheduleRequest`]
#[derive(Debug, Default)]
pub struct ScheduleRequestBuilder {
    request_type: Option<RequestType>,
    group_id: Option<String>,
    teacher_id: Option<String>,
    start: Option<String>,
    end: Option<String>
}
impl ScheduleRequestBuilder {
    pub fn new() -> Self {
        ScheduleRequestBuilder::default()
    }
    pub fn request_type(mut self, request_type: RequestType) -> Self {
        self.request_type = Some(request_type);
        self
    }
    pub fn group_id(mut self, group_id: &str) -> Self {
        self.group_id = Some(group_id.into());
        self
    }
    pub fn teacher_id(mut self, teacher_id: &str) -> Self {
        self.teacher_id = Some(teacher_id.into());
        self
    }
    pub fn start(mut self, start: String) -> Self {
        self.start = Some(start);
        self
    }
    pub fn end(mut self, end: String) -> Self {
        self.end = Some(end);
        self
    }
    pub fn build(mut self) -> Result<ScheduleRequest, ScheduleBuildErrors> {
        let req = ScheduleRequest {
            request_type: match self.request_type {
                Some(req) => req,
                None => return Err(ScheduleBuildErrors::NoRequestType),
            },
            group_id: self.group_id,
            teacher_id: self.teacher_id,
            start: match self.start {
                Some(start) => start,
                None => return Err(ScheduleBuildErrors::NoDate),
            },
            end: self.end,
        };
        Ok(req)
    }
}

impl ScheduleRequest {
    pub fn form_schedule_url(&self) -> Result<Url, ParseError> {
        let query = self.form_base64_query();
        let url_string = format!("https://www.chsu.ru/raspisanie/cache/{}.json", query);
        url_string.parse::<reqwest::Url>()
    }
    fn form_base64_query(&self) -> String {
        let request_type = match &self.request_type {
            RequestType::Student => {"student"}
            RequestType::Teacher => {"tutor"}
        };
        let group_id = match &self.group_id {
            Some(group) => format!("\"{}\"", group),
            None => "null".to_owned(),
        };
        let teacher_id = match &self.teacher_id {
            Some(teacher) => format!("\"{}\"", teacher),
            None => "null".to_owned()
        };
        let start: &String = &self.start;
        let end = match &self.end {
            Some(date) => date,
            None => start
        };

        let query = format!("[\"{}\",{},{},\"{}\",\"{}\"]", request_type, group_id, teacher_id, start, end);
        base64::engine::general_purpose::STANDARD.encode(query)
    }
}

#[cfg(test)]
mod builder_tests {
    use crate::schedule::request::{RequestType, ScheduleRequest, ScheduleRequestBuilder};
    #[test]
    fn test_correct_builder() {
        let test_request: ScheduleRequest = ScheduleRequestBuilder::new()
            .request_type(RequestType::Student)
            .group_id("1739582424505775711")
            .teacher_id("1472314025600620405")
            .start("06.02.2024".into())
            .end("06.02.2024".into())
            .build()
            .unwrap();
        let correct_request = ScheduleRequest {
            request_type: RequestType::Student,
            group_id: Some("1739582424505775711".into()),
            teacher_id: Some("1472314025600620405".into()),
            start: "06.02.2024".into(),
            end: Some("06.02.2024".into()),
        };
        assert_eq!(test_request, correct_request);
    }
    #[test]
    #[should_panic]
    fn test_missing_request_type() {
        let test_request: ScheduleRequest = ScheduleRequestBuilder::new()
            .group_id("1739582424505775711")
            .teacher_id("1472314025600620405")
            .start("06.02.2024".into())
            .end("06.02.2024".into())
            .build()
            .unwrap();
    }
    #[test]
    #[should_panic]
    fn test_missing_date() {
        let test_request: ScheduleRequest = ScheduleRequestBuilder::new()
            .request_type(RequestType::Student)
            .group_id("1739582424505775711")
            .teacher_id("1472314025600620405")
            .end("06.02.2024".into())
            .build()
            .unwrap();
    }
}
#[cfg(test)]
mod url_tests {
    use url::Url;
    use crate::schedule::request::{RequestType, ScheduleRequestBuilder};

    #[test]
    fn test_url_correctness() {
        let correct_url: Url = "https://www.chsu.ru/raspisanie/cache/WyJzdHVkZW50IiwiMTczOTU4MjQyNDUwNTc3NTcxMSIsIjE0NzIzMTQwMjU2MDA2MjA0MDUiLCIwNi4wMi4yMDI0IiwiMDYuMDIuMjAyNCJd.json".parse().unwrap();
        let test_url: Url = ScheduleRequestBuilder::new()
            .request_type(RequestType::Student)
            .group_id("1739582424505775711")
            .teacher_id("1472314025600620405")
            .start("06.02.2024".into())
            .end("06.02.2024".into())
            .build()
            .unwrap()
            .form_schedule_url()
            .unwrap();
        assert_eq!(correct_url, test_url)
    }
}