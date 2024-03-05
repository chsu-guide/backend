use chrono::{DateTime, NaiveDateTime, Utc};

pub struct DbSchedule {
    pub id: u64,
    pub request_date: DateTime<Utc>,
    pub class_date: DateTime<Utc>,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub discipline_id: u64,
    pub lesson_type: Box<str>,
}