use unitracker_chsu::model::groups::Faculty as ApiFaculty;

pub struct DbFaculty {
    id: i64,
    title: Box<str>,
}

impl From<ApiFaculty> for DbFaculty {
    fn from(value: ApiFaculty) -> Self {
        DbFaculty {
            id: value.id,
            title: value.title.into_boxed_str(),
        }
    }
}