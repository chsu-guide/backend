use serde_derive::Deserialize;

pub type TeacherList = Vec<Teacher>;
#[derive(Debug, Default, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Teacher {
    /// Numerical ID for db
    pub id: u64,
    pub last_name: String,
    pub first_name: String,
    pub middle_name: Option<String>,
    /// Last name + initials
    pub short_name: String,
    /// Last + first + third names as one string
    pub fio: String,
}
