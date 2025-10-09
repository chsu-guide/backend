use unitracker_chsu::model::disciplines;

#[derive(Debug)]
pub struct Discipline {
    pub id: i64,
    pub name: Box<str>,
}

impl From<disciplines::Discipline> for Discipline {
    fn from(value: disciplines::Discipline) -> Self {
        Self {
            id: value.id,
            name: value.title.into_boxed_str(),
        }
    }
}
