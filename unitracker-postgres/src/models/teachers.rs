use unitracker_chsu::model::teachers::Teacher as ApiTeacher;

#[derive(Debug)]
pub struct DbTeacher {
    pub id: i64,
    pub last_name: Box<str>,
    pub first_name: Box<str>,
    pub middle_name: Option<String>
}

impl From<ApiTeacher> for DbTeacher {
    fn from(value: ApiTeacher) -> Self {
        Self {
            id: value.id,
            last_name: value.last_name.into_boxed_str(),
            first_name: value.first_name.into_boxed_str(),
            middle_name: Some(value.middle_name.unwrap_or("".to_string()))
            // middle_name: match value.middle_name {
            //     Some(n) => n.into_boxed_str(),
            //     None => "".to_string().into_boxed_str()
            // },
        }
    }
}