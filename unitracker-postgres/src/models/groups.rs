use unitracker_chsu::model::groups::Group as ApiGroup;
pub struct DbGroup {
    pub id: i64,
    pub title: Box<str>,
    pub course: i8,
    pub faculty_id: Option<i64>,
    pub chair_id: Option<i64>,
}

impl From<ApiGroup> for DbGroup {
    fn from(value: ApiGroup) -> Self {
        Self {
            id: value.id,
            title: value.title.into_boxed_str(),
            course: value.course,
            faculty_id: Some(value.faculty.id),
            chair_id: Some(value.chair.id),
        }
    }
}