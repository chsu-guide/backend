use sqlx::prelude::FromRow;
use unitracker_chsu::model::teachers;

#[derive(Debug, FromRow)]
pub struct Teacher {
    pub id: i64,
    pub last_name: Box<str>,
    pub first_name: Box<str>,
    pub middle_name: Option<String>,
}

impl From<teachers::Teacher> for Teacher {
    fn from(value: teachers::Teacher) -> Self {
        Self {
            id: value.id,
            last_name: value.last_name.into_boxed_str(),
            first_name: value.first_name.into_boxed_str(),
            middle_name: value.middle_name,
        }
    }
}
