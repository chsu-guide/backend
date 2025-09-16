use crate::model::schedule::Schedule;
use crate::request::constants::{BASE_URL, TIMETABLE};
use crate::request::util::{call_with_url, check_result};
use crate::request::RequestErrors;
use chrono::NaiveDate;
use std::fmt::{Display, Formatter};
use url::Url;

pub async fn get_school_week(date: NaiveDate) -> Result<usize, RequestErrors> {
    let week_url =
        BASE_URL.to_owned() + TIMETABLE + "/numweek/" + &date.format("%d.%m.%Y").to_string() + "/";
    let response = call_with_url(&week_url).await?;
    check_result(response).await
}

pub async fn get_schedule(
    schedule_request: ScheduleRequest,
) -> Result<Vec<Schedule>, RequestErrors> {
    let schedule_url: String = schedule_request.into();
    let response = call_with_url(&schedule_url).await?;
    check_result(response).await
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
        let mut base = BASE_URL.to_owned() + TIMETABLE;
        match &self.schedule_type {
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
                    + &g.to_string())
            }
            ScheduleType::Lecturer(l) => {
                base += &("/from/".to_owned()
                    + &self.start.format("%d.%m.%Y").to_string()
                    + "/to/"
                    + &self.end.format("%d.%m.%Y").to_string()
                    + "/lecturerId/"
                    + &l.to_string())
            }
        }
        write!(f, "{}", base)
    }
}

impl Into<String> for ScheduleRequest {
    fn into(self) -> String {
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
                    + &g.to_string())
            }
            ScheduleType::Lecturer(l) => {
                base += &("/from/".to_owned()
                    + &self.start.format("%d.%m.%Y").to_string()
                    + "/to/"
                    + &self.end.format("%d.%m.%Y").to_string()
                    + "/lecturerId/"
                    + &l.to_string())
            }
        }
        base
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
                    + &g.to_string())
            }
            ScheduleType::Lecturer(l) => {
                base += &("/from/".to_owned()
                    + &self.start.format("%d.%m.%Y").to_string()
                    + "/to/"
                    + &self.end.format("%d.%m.%Y").to_string()
                    + "/lecturerId/"
                    + &l.to_string())
            }
        }
        Url::parse(&base).unwrap()
    }
}
