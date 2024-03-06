use chrono::{DateTime, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use unitracker_chsu::model::schedule::Class as ApiSchedule;

#[derive(Debug)]
pub struct DbSchedule {
    pub id: i64,
    pub request_date: NaiveDate,
    pub class_date: NaiveDate,
    pub start_time: NaiveTime,
    pub end_time: NaiveTime,
    pub lesson_type: Box<str>,
    pub lesson_type_abbr: Option<String>,
    pub discipline_id: i64,
}

impl From<ApiSchedule> for DbSchedule {
    fn from(value: ApiSchedule) -> Self {
        Self {
            id: value.id,
            request_date: Utc::now().naive_local().date(),
            class_date: match NaiveDate::parse_from_str(&value.date_event, "%d.%m.%Y"){
                Ok(date) => date,
                Err(e) => panic!("{:#?}, error: {e}, groups: {:?}, disc: {}", value.date_event.into_bytes(),  value.groups, value.discipline.title),
            },
            start_time: value.start_time.parse().unwrap(),
            end_time: value.end_time.parse().unwrap(),
            lesson_type: value.lessontype.unwrap().into_boxed_str(),
            lesson_type_abbr: value.abbrlessontype,
            discipline_id: value.discipline.id,
        }
    }
}