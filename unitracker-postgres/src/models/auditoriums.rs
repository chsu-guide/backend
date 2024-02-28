use unitracker_chsu::model::auditoriums::Auditorium as ApiAuditorium
pub struct DbAuditorium {
    id: i64,
    name: Box<str>,
    number: Box<str>,
    height: f64,
    length: f64,
    width: f64,
    building_id: i64,
}

impl From<ApiAuditorium> for DbAuditorium {
    fn from(value: ApiAuditorium) -> Self {
        Self {
            id: value.id,
            name: Box::into(value.name.into()),
            number: Box::into(value.number.into()),
            height: value.height,
            length: value.length,
            width: value.width,
            building_id: value.build_id,
        }
    }
}