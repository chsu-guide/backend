use unitracker_chsu::model::teachers::Teacher as ApiTeacher;

pub struct DbTeacher {
    id: i64,
    last_name: Box<str>,
    first_name: Box<str>,
    middle_name: Option<Box<str>>
}

impl From<ApiTeacher> for DbTeacher {
    fn from(value: ApiTeacher) -> Self {
        Self {
            id: value.id,
            last_name: Box::new(value.last_name.into()),
            first_name: Box::new(value.first_name.into()),
            middle_name: *match value.middle_name {
                Some(n) => Box::new(n.into()),
                None => None
            },
        }
    }
}