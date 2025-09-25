use crate::ChsuClient;
use crate::model::schedule::Schedule;
use crate::request::RequestErrors;
use crate::request::constants::TIMETABLE_URL;
use crate::request::util::format_date;
use crate::utils::response::ToConcrete;
use chrono::NaiveDate;
use std::fmt::{Display, Formatter};
use url::Url;

impl ChsuClient {
    pub async fn get_school_week(&self, date: NaiveDate) -> Result<usize, RequestErrors> {
        let week_url = format!("{}/numweek/{}/", TIMETABLE_URL, format_date(date));
        self.call_with_url(&week_url).to_concrete().await
    }
    pub async fn get_schedule(
        &self,
        schedule_request: ScheduleRequest,
    ) -> Result<Vec<Schedule>, RequestErrors> {
        let schedule_url: String = schedule_request.to_string();
        self.call_with_url(&schedule_url).to_concrete().await
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
                format!("{TIMETABLE_URL}/event/from/{start_fmt}/to/{end_fmt}")
            }
            ScheduleType::Group(g) => {
                format!("{TIMETABLE_URL}/from/{start_fmt}/to/{end_fmt}/groupId/{g}")
            }
            ScheduleType::Lecturer(l) => {
                format!("{TIMETABLE_URL}/from/{start_fmt}/to/{end_fmt}/lecturerId/{l}")
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
