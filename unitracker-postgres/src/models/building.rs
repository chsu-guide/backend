use unitracker_chsu::model::buildings::Building as ApiBuilding;

#[derive(Debug)]
pub struct DbBuilding {
    pub id: i64,
    pub title: Box<str>,
}

impl From<ApiBuilding> for DbBuilding {
    fn from(value: ApiBuilding) -> Self {
        Self {
            id: value.id,
            title: value.title.into_boxed_str()
        }
    }
}