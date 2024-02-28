use chrono::{NaiveDate, NaiveDateTime};
use reqwest::{ClientBuilder, Method, StatusCode};
use url::Url;
use crate::model::groups::Group;
use crate::model::schedule::{Class};
use crate::model::teachers::Teacher;
use crate::request::constants::{BASE_URL, TIMETABLE};
use crate::request::RequestErrors;

pub async fn get_schedule(bearer: &str, schedule_request: ScheduleRequest) -> Result<Vec<Class>, RequestErrors> {
    let schedule_url: Url = schedule_request.into();
    let client = ClientBuilder::new().user_agent("").build()?;
    let mut response = client
        .request(Method::GET, schedule_url)
        .header("content-type", "application/json")
        .bearer_auth(bearer)
        .send()
        .await?;
    let response_result = match response.status() {
        StatusCode::OK => Ok(response.json().await?),
        StatusCode::UNAUTHORIZED => Err(RequestErrors::InvalidBearerToken),
        _ => Err(RequestErrors::UnknownError)
    };
    response_result
}

pub enum ScheduleType {
    Full,
    Group(Group),
    Lecturer(Teacher)
}

pub struct ScheduleRequest {
    start: NaiveDate,
    end: NaiveDate,
    schedule_type: ScheduleType
}
pub struct ScheduleRequestBuilder {
    start: Option<NaiveDate>,
    end: Option<NaiveDate>,
    schedule_type: Option<ScheduleType>
}
impl ScheduleRequestBuilder {
    pub fn new() -> Self {
        ScheduleRequestBuilder {
            start: None,
            end: None,
            schedule_type: None,
        }
    }
    #[must_use = "start date is required"]
    pub fn start(mut self, date: NaiveDate) -> Self {
        self.start = Some(date);
        self
    }
    #[must_use = "end date is required"]
    pub fn end(mut self, date: NaiveDate) -> Self {
        self.end = Some(date);
        self
    }
    #[must_use = "schedule type is required"]
    pub fn schedule_type(mut self, schedule_type: ScheduleType) -> Self {
        self.schedule_type = Some(schedule_type);
        self
    }
    pub fn build(mut self) -> ScheduleRequest {
        ScheduleRequest {
            start: self.start.unwrap(),
            end: self.end.unwrap(),
            schedule_type: self.schedule_type.unwrap()
        }
    }
}

impl Into<Url> for ScheduleRequest {
    fn into(self) -> Url {
        let mut base = BASE_URL.to_owned() + TIMETABLE;
        match self.schedule_type {
            ScheduleType::Full => {
                base += &("/event/from/".to_owned()
                    + &self.start.format("%d.%m.%Y").to_string()
                    + "/to/"
                    + &self.end.format("%d.%m.%Y").to_string())
            }
            ScheduleType::Group(g) => {
                base += &("/from/".to_owned()
                    + &self.start.format("%d.%m.%Y").to_string()
                    + "/to/"
                    + &self.end.format("%d.%m.%Y").to_string()
                    + "/groupId/"
                    + &g.id.to_string())
            }
            ScheduleType::Lecturer(l) => {
                base += &("/from/".to_owned()
                    + &self.start.format("%d.%m.%Y").to_string()
                    + "/to/"
                    + &self.end.format("%d.%m.%Y").to_string()
                    + "/lecturerId/"
                    + &l.id.to_string())
            }
        }
        Url::parse(&base).unwrap()
    }
}