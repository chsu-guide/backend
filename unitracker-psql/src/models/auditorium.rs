use unitracker_chsu::model::auditoriums;
pub struct Auditorium {
    pub id: i64,
    pub name: Box<str>,
    pub number: Box<str>,
    pub building_id: Option<i64>,
}

impl From<auditoriums::Auditorium> for Auditorium {
    fn from(value: auditoriums::Auditorium) -> Self {
        Self {
            id: value.id,
            name: value.name.into_boxed_str(),
            number: value.number.into_boxed_str(),
            building_id: Some(value.building_id),
        }
    }
}
