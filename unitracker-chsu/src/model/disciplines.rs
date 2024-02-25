use serde_derive::Deserialize;

pub type DisciplineList = Vec<Discipline>;
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Discipline {
    pub id: i64,
    pub title: String,
}