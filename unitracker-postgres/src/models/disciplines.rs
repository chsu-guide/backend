use unitracker_chsu::model::disciplines::Discipline as ApiDiscipline;

pub struct DbDiscipline {
    pub id: i64,
    pub title: Box<str>,
}

impl From<ApiDiscipline> for DbDiscipline {
    fn from(value: ApiDiscipline) -> Self {
        Self {
            id: value.id,
            title: value.title.into_boxed_str(),
        }
    }
}