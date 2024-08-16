use unitracker_chsu::model::groups::Chair as ApiChair;

pub struct DbChair {
    id: i64,
    title: Box<str>,
}

impl From<ApiChair> for DbChair {
    fn from(value: ApiChair) -> Self {
        DbChair {
            id: value.id,
            title: value.title.into_boxed_str(),
        }
    }
}