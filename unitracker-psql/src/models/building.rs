use unitracker_chsu::model::buildings;

#[derive(Debug)]
pub struct Building {
    pub id: i64,
    pub name: Box<str>,
}

impl From<&buildings::Building> for Building {
    fn from(value: &buildings::Building) -> Self {
        Self {
            id: value.id,
            name: value.title.clone().into_boxed_str(),
        }
    }
}
