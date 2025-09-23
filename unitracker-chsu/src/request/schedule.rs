use crate::model::schedule::Schedule;
use crate::request::constants::{BASE_URL, TIMETABLE};
use crate::request::util::{check_result, format_date};
use crate::request::RequestErrors;
use crate::ChsuClient;
use chrono::NaiveDate;
use std::fmt::{Display, Formatter};
use url::Url;

impl ChsuClient {
    pub async fn get_school_week(&self, date: NaiveDate) -> Result<usize, RequestErrors> {
        let week_url = format!("{}{}/numweek/{}/", BASE_URL, TIMETABLE, format_date(date));
        let response = self.call_with_url(&week_url).await?;
        check_result(response).await
    }
    pub async fn get_schedule(
        &self,
        schedule_request: ScheduleRequest,
    ) -> Result<Vec<Schedule>, RequestErrors> {
        let schedule_url: String = schedule_request.to_string();
        let response = self.call_with_url(&schedule_url).await?;
        check_result(response).await
    }
}

pub enum ScheduleType {
    Full,
    Group(u64),
    Lecturer(u64),
}

pub struct ScheduleRequest {
    start: NaiveDate,
    end: NaiveDate,
    schedule_type: ScheduleType,
}
pub struct ScheduleRequestBuilder {
    start: Option<NaiveDate>,
    end: Option<NaiveDate>,
    schedule_type: Option<ScheduleType>,
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
    pub fn build(self) -> ScheduleRequest {
        ScheduleRequest {
            start: self.start.unwrap(),
            end: self.end.unwrap(),
            schedule_type: self.schedule_type.unwrap(),
        }
    }
}

impl Display for ScheduleRequest {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let str: String = self.into();
        write!(f, "{}", str)
    }
}
impl From<&ScheduleRequest> for String {
    fn from(val: &ScheduleRequest) -> Self {
        let start_fmt = format_date(val.start);
        let end_fmt = format_date(val.end);
        let base = match &val.schedule_type {
            ScheduleType::Full => {
                format!("{BASE_URL}{TIMETABLE}/event/from/{start_fmt}/to/{end_fmt}")
            }
            ScheduleType::Group(g) => {
                format!("{BASE_URL}{TIMETABLE}/from/{start_fmt}/to/{end_fmt}/groupId/{g}")
            }
            ScheduleType::Lecturer(l) => {
                format!("{BASE_URL}{TIMETABLE}/from/{start_fmt}/to/{end_fmt}/lecturerId/{l}")
            }
        };
        base
    }
}

impl From<ScheduleRequest> for String {
    fn from(value: ScheduleRequest) -> Self {
        value.into()
    }
}

impl From<ScheduleRequest> for Url {
    fn from(val: ScheduleRequest) -> Self {
        Url::parse(&val.to_string()).unwrap()
    }
}
