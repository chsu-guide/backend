use unitracker_chsu::model::buildings;

pub struct Building {
    pub id: i64,
    pub name: Box<str>,
}

impl From<buildings::Building> for Building {
    fn from(value: buildings::Building) -> Self {
        Self {
            id: value.id,
            name: value.title.into_boxed_str(),
        }
    }
}
