use chrono::{self, NaiveDate, NaiveDateTime, NaiveTime, Utc};
use unitracker_chsu::model::{disciplines::Discipline, schedule::Schedule};

#[derive(Debug)]
pub struct Class {
    pub id: i64,
    pub created_at: NaiveDateTime,
    pub start_time: NaiveDateTime,
    pub end_time: NaiveDateTime,
    pub lesson_type: Box<str>,
    pub lesson_type_abbreviated: Option<String>,
    pub discipline_id: Option<i64>,
}

impl From<Schedule> for Class {
    fn from(value: Schedule) -> Self {
        let date = NaiveDate::parse_from_str(&value.date_event, "%d.%m.%Y").unwrap();
        Self {
            id: value.id,
            created_at: Utc::now().naive_local(),
            start_time: {
                let time: NaiveTime = value.start_time.parse().unwrap();
                NaiveDateTime::new(date, time)
            },
            end_time: {
                let time = value.end_time.parse().unwrap();
                NaiveDateTime::new(date, time)
            },
            lesson_type: value
                .lessontype
                .map(|s| s.into_boxed_str())
                .unwrap_or_default(),
            lesson_type_abbreviated: value.abbrlessontype,
            discipline_id: value
                .discipline
                .map(|d| d.id)
                .and_then(|d| if d == 0 { None } else { Some(d) }),
        }
    }
}
