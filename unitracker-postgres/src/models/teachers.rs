use unitracker_chsu::model::teachers::Teacher as ApiTeacher;

pub struct DbTeacher {
    id: u64,
    last_name: Box<str>,
    first_name: Box<str>,
    middle_name: Box<str>
}

impl From<ApiTeacher> for DbTeacher {
    fn from(value: ApiTeacher) -> Self {
        Self {
            id: value.id,
            last_name: value.last_name.into_boxed_str(),
            first_name: value.first_name.into_boxed_str(),
            middle_name: match value.middle_name {
                Some(n) => n.into_boxed_str(),
                None => "".to_string().into_boxed_str()
            },
        }
    }
}