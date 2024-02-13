use serde_derive::Deserialize;

pub type BuildingList = Vec<Building>;
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Building {
    pub id: i64,
    pub title: String,
}
