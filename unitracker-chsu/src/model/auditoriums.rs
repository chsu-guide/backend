use serde_derive::Deserialize;

pub type AuditoriumList = Vec<Auditorium>;
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Auditorium {
    pub id: i64,
    pub name: String,
    pub number: String,
    pub build_name: String,
    pub build_id: i64,
    pub height: f32,
    pub length: f32,
    pub width: f32,
}