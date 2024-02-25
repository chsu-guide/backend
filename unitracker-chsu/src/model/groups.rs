use serde_derive::Deserialize;

pub type GroupList = Vec<Group>;
/// Information about the study group
#[derive(Debug, Default, Clone, Deserialize)]
pub struct Group {
    /// group ID for db
    pub id: i64,
    pub title: String,
    /// Study year
    pub course: i64,
    /// Study faculty
    pub faculty: Faculty,
    /// Which chair the faculty is from
    pub chair: Chair,
}
#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Faculty {
    pub id: i64,
    pub title: String,
}

#[derive(Default, Debug, Clone, PartialEq, Deserialize)]
pub struct Chair {
    pub id: i64,
    pub title: String,
}
