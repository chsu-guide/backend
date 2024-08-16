use serde_derive::Deserialize;

pub type AuditoriumList = Vec<Auditorium>;
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Auditorium {
    pub id: i64,
    pub name: String,
    pub number: String,
    #[serde(alias = "buildName")]
    pub building_name: String,
    #[serde(alias = "buildId")]
    pub building_id: i64,
    pub height: f32,
    pub length: f32,
    pub width: f32,
}