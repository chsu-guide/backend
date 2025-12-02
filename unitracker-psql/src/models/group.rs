use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use unitracker_chsu::model::groups;
#[derive(Debug, FromRow)]
pub struct Group {
    pub id: i64,
    pub name: Box<str>,
    pub course: i16, // limited to 1-6 I think
    pub faculty_id: Option<i64>,
    pub chair_id: Option<i64>,
}

#[derive(Debug, FromRow, Deserialize, Serialize)]
pub struct GroupShort {
    pub id: i64,
    pub name: Box<str>,
    pub course: i16,
}

impl From<groups::Group> for Group {
    fn from(value: groups::Group) -> Self {
        Self {
            id: value.id,
            name: value.title.into_boxed_str(),
            course: value.course as i16,
            faculty_id: Some(value.faculty.id),
            chair_id: Some(value.chair.id),
        }
    }
}
