use serde_derive::Deserialize;

pub type AuditoriumList = Vec<Auditorium>;
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Auditorium {
    pub id: i64,
    pub name: String,
    pub number: String,
    pub build_name: String,
    pub build_id: i64,
    pub height: f64,
    pub length: f64,
    pub width: f64,
}