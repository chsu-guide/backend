use unitracker_chsu::model::auditoriums::Auditorium as ApiAuditorium;
#[derive(Debug)]
pub struct DbAuditorium {
    pub id: i64,
    pub name: Box<str>,
    pub number: Box<str>,
    pub height: f32,
    pub length: f32,
    pub width: f32,
    pub building_id: Option<i64>,
}

impl From<ApiAuditorium> for DbAuditorium {
    fn from(value: ApiAuditorium) -> Self {
        Self {
            id: value.id,
            name: value.name.into_boxed_str(),
            number: value.number.into_boxed_str(),
            height: value.height,
            length: value.length,
            width: value.width,
            building_id: Some(value.building_id),
        }
    }
}